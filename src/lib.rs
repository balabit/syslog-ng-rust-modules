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

pub mod options;
pub mod logevent;
pub mod mock;
pub mod logtemplate;

pub const CLASSIFIER_UUID: &'static str = ".classifier.uuid";
pub const CLASSIFIER_CLASS: &'static str = ".classifier.class";

pub struct CorrelationParserBuilder<P, E, T, TF> where P: Pipe, E: 'static + Event, T: 'static + Template<Event=E>, TF: TemplateFactory<E, Template=T> {
    contexts: Option<Correlator<E, T>>,
    formatter: MessageFormatter,
    template_factory: TF,
    delta: Option<Duration>,
    _marker: PhantomData<(P, E, T, TF)>
}

impl<P, E, T, TF> CorrelationParserBuilder<P, E, T, TF> where P: Pipe, E: Event, T: Template<Event=E>, TF: TemplateFactory<E, Template=T> {
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

impl<P, E, T, TF> ParserBuilder<P> for CorrelationParserBuilder<P, E, T, TF> where P: Pipe, E: 'static + Event + Into<LogMessage>, T: 'static + Template<Event=E>, TF: TemplateFactory<E, Template=T> + From<GlobalConfig> {
    type Parser = CorrelationParser<E, T>;
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
        let _ = delta;
        let contexts = try!(contexts.ok_or(OptionError::missing_required_option(options::CONTEXTS_FILE)));
        Ok(CorrelationParser::new(contexts, formatter))
    }
}

pub struct CorrelationParser<E: 'static + Event, T: 'static + Template<Event=E>> {
    correlator: Arc<Mutex<Correlator<E, T>>>,
    formatter: MessageFormatter,
}

impl<E, T> Clone for CorrelationParser<E, T> where E: Event, T: Template<Event=E> {
    fn clone(&self) -> CorrelationParser<E, T> {
        CorrelationParser { correlator: self.correlator.clone(), formatter: self.formatter.clone() }
    }
}

impl<E, T> CorrelationParser<E, T> where E: Event, T: Template<Event=E> {
    pub fn new(correlator: Correlator<E, T>, formatter: MessageFormatter) -> CorrelationParser<E, T> {
        CorrelationParser {
            correlator: Arc::new(Mutex::new(correlator)),
            formatter: formatter,
        }
    }
    fn on_alert<P>(guard: &mut MutexGuard<Correlator<E, T>>, alert: Alert<E>, parent: &mut P)
        where P: Pipe, E: Into<LogMessage> {
        match alert.inject_mode {
            InjectMode::Log => {
                debug!("LOG: {}", alert.message.message());
            },
            InjectMode::Forward => {
                debug!("FORWARD: {}", alert.message.message());
                let logmsg = alert.message.into();
                parent.forward(logmsg);
            },
            InjectMode::Loopback => {
                debug!("LOOPBACK: {}", alert.message.message());
                guard.push_message(alert.message);
            },
        }
    }
}

impl<P, E, T> Parser<P> for CorrelationParser<E, T> where P: Pipe, E: Event + Into<LogMessage>, T: Template<Event=E> {
    fn parse(&mut self, parent: &mut P, msg: &mut LogMessage, message: &str) -> bool {
        debug!("CorrelationParser: process()");
        let message = {
            if let Some(uuid) = msg.get(CLASSIFIER_UUID) {
                let name = msg.get(CLASSIFIER_CLASS);

                let mut event = E::new(uuid, message);
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
                    CorrelationParser::on_alert(&mut guard, alert, parent);
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

parser_plugin!(CorrelationParserBuilder<LogParser, LogEvent, LogTemplate, LogTemplateFactory>);
