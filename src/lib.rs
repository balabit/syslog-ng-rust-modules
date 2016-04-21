#[macro_use]
extern crate log;
#[macro_use]
extern crate syslog_ng_common;
extern crate correlation;

use correlation::{Alert, Event, Template, TemplateFactory};
use correlation::config::action::message::InjectMode;
use correlation::correlator::{Correlator, CorrelatorFactory};
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard};
use std::error::Error;
use std::time::Duration;
use std::str::FromStr;
use syslog_ng_common::{MessageFormatter, LogMessage};
use syslog_ng_common::{Parser, ParserBuilder, OptionError, Pipe, GlobalConfig};

pub use logevent::LogEvent;
pub use logtemplate::{LogTemplate, LogTemplateFactory};
pub use timer::Watchdog;

pub mod options;
pub mod logevent;
pub mod mock;
pub mod logtemplate;
pub mod timer;

pub const CLASSIFIER_UUID: &'static [u8] = b".classifier.uuid";
pub const CLASSIFIER_CLASS: &'static [u8] = b".classifier.class";

pub trait Timer<E, T> where E: Event + Send, T: Template<Event=E> {
    fn new(delta: Duration, correlator: Arc<Mutex<Correlator<E, T>>>) -> Self;
}

pub struct CorrelationParserBuilder<P, E, T, TF, TM> where P: Pipe, E: 'static + Event + Send, T: 'static + Template<Event=E>, TF: TemplateFactory<E, Template=T>, TM: Timer<E, T> {
    contexts: Option<Correlator<E, T>>,
    formatter: MessageFormatter,
    template_factory: TF,
    delta: Option<Duration>,
    _marker: PhantomData<(P, E, T, TF, TM)>
}

impl<P, E, T, TF, TM> CorrelationParserBuilder<P, E, T, TF, TM> where P: Pipe, E: Event + Send, T: Template<Event=E>, TF: TemplateFactory<E, Template=T>, TM: Timer<E, T> {
    pub fn set_file(&mut self, path: &str) {
        match CorrelatorFactory::from_path::<T, &str, E, TF>(path, &self.template_factory) {
            Ok(correlator) => {
                self.contexts = Some(correlator);
            },
            Err(err) => {
                error!("Failed to initialize correlation-parser from configuration file: {}", &err);
                while let Some(err) = err.cause() {
                    info!("Error: {}", err.description());
                    info!("Cause: {}", &err);
                }
            }
        }
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.formatter.set_prefix(prefix);
    }

    pub fn set_delta(&mut self, delta: String) {
        match u64::from_str(&delta) {
            Ok(delta) => {
                info!("correlation-parser: using {} ms as delta time between timer events", &delta);
                self.delta = Some(Duration::from_millis(delta));
            },
            Err(err) => error!("{}", err)
        }
    }
}

impl<P, E, T, TF, TM> ParserBuilder<P> for CorrelationParserBuilder<P, E, T, TF, TM> where P: Pipe, E: 'static + Event + Into<LogMessage> + Send, T: 'static + Template<Event=E>, TF: TemplateFactory<E, Template=T> + From<GlobalConfig>, TM: Timer<E, T> {
    type Parser = CorrelationParser<E, T, TM>;
    fn new(cfg: GlobalConfig) -> Self {
        CorrelationParserBuilder {
            contexts: None,
            formatter: MessageFormatter::new(),
            template_factory: TF::from(cfg),
            delta: Some(Duration::from_millis(1000)),
            _marker: PhantomData
        }
    }
    fn option(&mut self, name: String, value: String) {
        debug!("CorrelationParser: set_option(key={}, value={})", &name, &value);

        match name.borrow() {
            options::CONTEXTS_FILE => self.set_file(&value),
            options::PREFIX => self.set_prefix(value),
            options::DELTA => self.set_delta(value),
            _ => debug!("CorrelationParser: not supported key: {:?}", name)
        };
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        debug!("Building CorrelationParser");
        let CorrelationParserBuilder {contexts, template_factory, formatter, delta, _marker } = self;
        let _ = template_factory;
        let contexts = try!(contexts.ok_or(OptionError::missing_required_option(options::CONTEXTS_FILE)));
        let delta = try!(delta.ok_or(OptionError::missing_required_option(options::DELTA)));
        let correlator = Arc::new(Mutex::new(contexts));
        let timer = Arc::new(TM::new(delta, correlator.clone()));
        Ok(CorrelationParser::new(correlator, formatter, delta, timer))
    }
}

pub struct CorrelationParser<E, T, TM> where E: 'static + Event + Send, T: 'static + Template<Event=E>, TM: Timer<E, T> {
    correlator: Arc<Mutex<Correlator<E, T>>>,
    delta: Duration,
    formatter: MessageFormatter,
    pub timer: Arc<TM>
}

impl<E, T, TM> Clone for CorrelationParser<E, T, TM> where E: Event + Send, T: Template<Event=E>, TM: Timer<E, T> {
    fn clone(&self) -> CorrelationParser<E, T, TM> {
        CorrelationParser {
            correlator: self.correlator.clone(),
            formatter: self.formatter.clone(),
            delta: self.delta.clone(),
            timer: self.timer.clone()
        }
    }
}

impl<E, T, TM> CorrelationParser<E, T, TM> where E: Event + Send, T: Template<Event=E>, TM: Timer<E, T> {
    pub fn new(correlator: Arc<Mutex<Correlator<E, T>>>, formatter: MessageFormatter, delta: Duration, timer: Arc<TM>) -> CorrelationParser<E, T, TM> {
        CorrelationParser {
            correlator: correlator,
            formatter: formatter,
            delta: delta,
            timer: timer
        }
    }
    fn on_alert<P>(guard: &mut MutexGuard<Correlator<E, T>>, alert: Alert<E>, parent: &mut P)
        where P: Pipe, E: Into<LogMessage> {
        match alert.inject_mode {
            InjectMode::Log => {
                debug!("LOG: {}", String::from_utf8_lossy(alert.message.message()));
            },
            InjectMode::Forward => {
                debug!("FORWARD: {}", String::from_utf8_lossy(alert.message.message()));
                let logmsg = alert.message.into();
                parent.forward(logmsg);
            },
            InjectMode::Loopback => {
                debug!("LOOPBACK: {}", String::from_utf8_lossy(alert.message.message()));
                guard.push_message(alert.message);
            },
        }
    }
}

impl<P, E, T, TM> Parser<P> for CorrelationParser<E, T, TM> where P: Pipe, E: Event + Into<LogMessage> + Send, T: Template<Event=E>, TM: Timer<E, T> {
    fn parse(&mut self, parent: &mut P, msg: &mut LogMessage, message: &str) -> bool {
        debug!("CorrelationParser: process()");
        let message = {
            if let Some(uuid) = msg.get(CLASSIFIER_UUID) {
                let name = msg.get(CLASSIFIER_CLASS);

                let mut event = E::new(uuid, message.as_bytes());
                for (k, v) in msg.values() {
                    event.set(&k, &v);
                }
                event.set_name(name);
                event
            } else {
                return false;
            }
        };

        match self.correlator.lock() {
            Ok(mut guard) => {
                guard.push_message(message);
                while let Some(alert) = guard.responses.pop_front() {
                    CorrelationParser::<E, T, TM>::on_alert(&mut guard, alert, parent);
                }
                true
            },
            Err(err) => {
                error!("{}", err);
                false
            }
        }
    }
}

parser_plugin!(CorrelationParserBuilder<LogParser, LogEvent, LogTemplate, LogTemplateFactory, Watchdog>);
