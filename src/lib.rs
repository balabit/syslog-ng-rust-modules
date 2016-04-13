#[macro_use]
extern crate log;
#[macro_use]
extern crate syslog_ng_common;
extern crate correlation;

use correlation::{Request, ContextMap, Alert, Event};
use correlation::config::action::message::InjectMode;
use correlation::correlator::{Correlator, AlertHandler, CorrelatorFactory};
use correlation::config::ContextConfig;
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::sync::{mpsc, Arc, Mutex};
use syslog_ng_common::{MessageFormatter, LogMessage};
use syslog_ng_common::{Parser, ParserBuilder, OptionError, Pipe};

pub use logevent::LogEvent;
pub use logtemplate::{LogTemplate, LogTemplateFactory};

pub mod options;
pub mod logevent;
pub mod mock;
pub mod logtemplate;

pub const CLASSIFIER_UUID: &'static str = ".classifier.uuid";
pub const CLASSIFIER_CLASS: &'static str = ".classifier.class";

struct MessageSender;

impl<P, E> AlertHandler<P, E> for MessageSender where P: Pipe, E: Event + Into<LogMessage> {
    fn on_alert(&mut self, alert: Alert<E>, reactor_input_channel: &mut mpsc::Sender<Request<E>>, parent: &mut P) {
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
                if let Err(err) = reactor_input_channel.send(Request::Message(Arc::new(alert.message))) {
                    error!("{}", err);
                }
            },
        }
    }
}

pub struct CorrelationParserBuilder<P, E> where P: Pipe, E: Event {
    contexts: Option<Vec<ContextConfig>>,
    formatter: MessageFormatter,
    _marker: PhantomData<(P, E)>
}

impl<P, E> CorrelationParserBuilder<P, E> where P: Pipe, E: Event {
    pub fn set_file(&mut self, path: &str) {
        match CorrelatorFactory::load_file(path) {
            Ok(contexts) => {
                self.contexts = Some(contexts);
            },
            Err(err) => {
                error!("CorrelationParser: failed to set config file: {:?}", &err);
            }
        }
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.formatter.set_prefix(prefix);
    }
}

impl<P, E> ParserBuilder<P> for CorrelationParserBuilder<P, E> where P: Pipe, E: 'static + Event + Into<LogMessage> {
    type Parser = CorrelationParser<P, E>;
    fn new() -> Self {
        CorrelationParserBuilder {
            contexts: None,
            formatter: MessageFormatter::new(),
            _marker: PhantomData
        }
    }
    fn option(&mut self, name: String, value: String) {
        debug!("CorrelationParser: set_option(key={}, value={})", &name, &value);

        match name.borrow() {
            "contexts_file" => self.set_file(&value),
            "prefix" => self.set_prefix(value),
            _ => debug!("CorrelationParser: not supported key: {:?}", name)
        };
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        debug!("Building CorrelationParser");
        let CorrelationParserBuilder {contexts, formatter, _marker } = self;
        let contexts = try!(contexts.ok_or(OptionError::missing_required_option(options::CONTEXTS_FILE)));
        let map = ContextMap::from_configs(contexts);
        let mut correlator: Correlator<P, E> = Correlator::new(map);
        correlator.set_alert_handler(Some(Box::new(MessageSender)));
        Ok(CorrelationParser::new(correlator, formatter))
    }
}

pub struct CorrelationParser<P: Pipe, E: 'static + Event> {
    correlator: Arc<Mutex<Correlator<P, E>>>,
    formatter: MessageFormatter,
}

impl<P, E> Clone for CorrelationParser<P, E> where P: Pipe, E: Event {
    fn clone(&self) -> CorrelationParser<P, E> {
        CorrelationParser { correlator: self.correlator.clone(), formatter: self.formatter.clone() }
    }
}

impl<P, E> CorrelationParser<P, E> where P: Pipe, E: Event {
    pub fn new(correlator: Correlator<P, E>, formatter: MessageFormatter) -> CorrelationParser<P, E> {
        CorrelationParser {
            correlator: Arc::new(Mutex::new(correlator)),
            formatter: formatter,
        }
    }
}

impl<P, E> Parser<P> for CorrelationParser<P, E> where P: Pipe, E: Event {
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
                guard.handle_events(parent);
                match guard.push_message(message) {
                    Ok(_) => true,
                    Err(err) => {
                        error!("{}", err);
                        false
                    }
                }
            },
            Err(err) => {
                error!("{}", err);
                false
            }
        }
    }
}

parser_plugin!(CorrelationParserBuilder<LogParser, LogEvent>);
