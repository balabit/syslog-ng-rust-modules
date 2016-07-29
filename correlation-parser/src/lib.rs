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
use std::time::Duration;
use std::str::FromStr;
use syslog_ng_common::{MessageFormatter, LogMessage};
use syslog_ng_common::{Parser, ParserBuilder, Pipe, GlobalConfig};
use syslog_ng_common::Error;

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
    fn start(&self) {}
    fn stop(&self) {}
}

pub trait TypeFamily {
    type Event: 'static + Event + Send;
    type Template: 'static + Template<Event=Self::Event>;
    type TemplateFactory: TemplateFactory<Self::Event, Template=Self::Template>;
    type Timer: Timer<Self::Event, Self::Template>;
}

pub struct SyslogNgTypeFamily {}

impl TypeFamily for SyslogNgTypeFamily {
    type Event = LogEvent;
    type Template = LogTemplate;
    type TemplateFactory = LogTemplateFactory;
    type Timer = Watchdog;
}

pub struct CorrelationParserBuilder<X: TypeFamily>  {
    correlator: Option<Arc<Mutex<Correlator<X::Event, X::Template>>>>,
    formatter: MessageFormatter,
    template_factory: Arc<X::TemplateFactory>,
    delta: Option<Duration>,
    _marker: PhantomData<X>
}

impl<X: TypeFamily> CorrelationParserBuilder<X> {
    pub fn set_file(&mut self, path: &str) -> Result<(), Error> {
        match CorrelatorFactory::from_path::<X::Template, &str, X::Event, X::TemplateFactory>(path, &self.template_factory) {
            Ok(correlator) => {
                let correlator = Arc::new(Mutex::new(correlator));
                self.correlator = Some(correlator);
                Ok(())
            },
            Err(err) => {
                let errmsg = format!("Failed to initialize correlation-parser from configuration file: {}", &err);
                Err(Error::verbatim_error(errmsg))
            }
        }
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.formatter.set_prefix(prefix);
    }

    pub fn set_delta(&mut self, delta: String) -> Result<(), Error> {
        match u64::from_str(&delta) {
            Ok(delta) => {
                info!("correlation-parser: using {} ms as delta time between timer events", &delta);
                self.delta = Some(Duration::from_millis(delta));
                Ok(())
            },
            Err(err) => {
                let errmsg = format!("{}", err);
                Err(Error::verbatim_error(errmsg))
            }
        }
    }
}

impl<X: TypeFamily> Clone for CorrelationParserBuilder<X> {
    fn clone(&self) -> Self {
        CorrelationParserBuilder {
            correlator: self.correlator.clone(),
            formatter: self.formatter.clone(),
            template_factory: self.template_factory.clone(),
            delta: self.delta.clone(),
            _marker: PhantomData
        }
    }
}

// TODO
impl<X: TypeFamily> ParserBuilder for CorrelationParserBuilder<X> where X::Event: Into<LogMessage>, X::TemplateFactory: From<GlobalConfig> {
    type Parser = CorrelationParser<X>;
    fn new(cfg: GlobalConfig) -> Self {
        CorrelationParserBuilder {
            correlator: None,
            formatter: MessageFormatter::new(),
            template_factory: Arc::new(X::TemplateFactory::from(cfg)),
            delta: Some(Duration::from_millis(1000)),
            _marker: PhantomData
        }
    }
    fn option(&mut self, name: String, value: String) -> Result<(), Error> {
        debug!("CorrelationParser: set_option(key={}, value={})", &name, &value);

        match name.borrow() {
            options::CONTEXTS_FILE => self.set_file(&value),
            options::PREFIX => {
                self.set_prefix(value);
                Ok(())
            },
            options::DELTA => self.set_delta(value),
            _ => Err(Error::unknown_option(name))
        }
    }
    fn build(self) -> Result<Self::Parser, Error> {
        debug!("Building CorrelationParser");
        let CorrelationParserBuilder {correlator, template_factory, formatter, delta, _marker } = self;
        let _ = template_factory;
        let correlator = try!(correlator.ok_or(Error::missing_required_option(options::CONTEXTS_FILE)));
        let delta = try!(delta.ok_or(Error::missing_required_option(options::DELTA)));
        let timer = Arc::new(X::Timer::new(delta, correlator.clone()));
        Ok(CorrelationParser::new(correlator, formatter, timer))
    }
}

pub struct CorrelationParser<X: TypeFamily> {
    correlator: Arc<Mutex<Correlator<X::Event, X::Template>>>,
    _formatter: MessageFormatter,
    pub timer: Arc<X::Timer>
}

impl<X: TypeFamily> CorrelationParser<X> {
    pub fn new(correlator: Arc<Mutex<Correlator<X::Event, X::Template>>>, formatter: MessageFormatter, timer: Arc<X::Timer>) -> CorrelationParser<X> {
        CorrelationParser {
            correlator: correlator,
            _formatter: formatter,
            timer: timer
        }
    }
    fn on_alert(guard: &mut MutexGuard<Correlator<X::Event, X::Template>>, alert: Alert<X::Event>, parent: &mut Pipe)
        where X::Event: Into<LogMessage> {
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

impl<X: TypeFamily> Parser for CorrelationParser<X>  where X::Event: Into<LogMessage> {
    fn parse(&mut self, parent: &mut Pipe, msg: &mut LogMessage, message: &str) -> bool {
        debug!("CorrelationParser: process()");
        let message = {
            if let Some(uuid) = msg.get(CLASSIFIER_UUID) {
                let name = msg.get(CLASSIFIER_CLASS);

                let mut event = X::Event::new(uuid, message.as_bytes());
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
                    CorrelationParser::<X>::on_alert(&mut guard, alert, parent);
                }
                true
            },
            Err(err) => {
                error!("{}", err);
                false
            }
        }
    }

    fn init(&mut self) -> bool {
        self.timer.start();
        true
    }

    fn deinit(&mut self) -> bool {
        self.timer.stop();
        true
    }
}

parser_plugin!(CorrelationParserBuilder<SyslogNgTypeFamily>);
