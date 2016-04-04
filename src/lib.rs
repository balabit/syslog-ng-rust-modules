#[macro_use]
extern crate log;
#[macro_use]
extern crate syslog_ng_common;
extern crate correlation;

use correlation::{Request, ContextMap, MessageBuilder, Alert};
use correlation::config::action::message::InjectMode;
use correlation::correlator::{Correlator, AlertHandler, CorrelatorFactory};
use correlation::config::ContextConfig;
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::sync::{mpsc, Arc, Mutex};
use syslog_ng_common::{MessageFormatter, LogMessage};
use syslog_ng_common::{Parser, ParserBuilder, OptionError, Pipe};

pub mod options;

pub const CLASSIFIER_UUID: &'static str = ".classifier.uuid";
pub const CLASSIFIER_CLASS: &'static str = ".classifier.class";

struct MessageSender;

impl<P> AlertHandler<P> for MessageSender where P: Pipe {
    fn on_alert(&mut self, alert: Alert, reactor_input_channel: &mut mpsc::Sender<Request>, parent: &mut P) {
        match alert.inject_mode {
            InjectMode::Log => {
                debug!("LOG: {}", alert.message.message());
            },
            InjectMode::Forward => {
                debug!("FORWARD: {}", alert.message.message());
                let message = alert.message;
                let mut logmsg = LogMessage::new();
                for (k, v) in message.values().iter() {
                    logmsg.insert(k.borrow(), v.borrow());
                }
                logmsg.insert("MESSAGE", message.message());
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

pub struct CorrelationParserBuilder<P: Pipe> {
    contexts: Option<Vec<ContextConfig>>,
    formatter: MessageFormatter,
    _marker: PhantomData<P>
}

impl<P: Pipe> CorrelationParserBuilder<P> {
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

impl<P: Pipe> ParserBuilder<P> for CorrelationParserBuilder<P> {
    type Parser = CorrelationParser<P>;
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
        let mut correlator: Correlator<P> = Correlator::new(map);
        correlator.set_alert_handler(Some(Box::new(MessageSender)));
        Ok(CorrelationParser::new(correlator, formatter))
    }
}

pub struct CorrelationParser<P: Pipe> {
    correlator: Arc<Mutex<Correlator<P>>>,
    formatter: MessageFormatter,
}

impl<P: Pipe> Clone for CorrelationParser<P> {
    fn clone(&self) -> CorrelationParser<P> {
        CorrelationParser { correlator: self.correlator.clone(), formatter: self.formatter.clone() }
    }
}

impl<P: Pipe> CorrelationParser<P> {
    pub fn new(correlator: Correlator<P>, formatter: MessageFormatter) -> CorrelationParser<P> {
        CorrelationParser {
            correlator: Arc::new(Mutex::new(correlator)),
            formatter: formatter,
        }
    }
}

impl<P: Pipe> Parser<P> for CorrelationParser<P> {
    fn parse(&mut self, parent: &mut P, msg: &mut LogMessage, message: &str) -> bool {
        debug!("CorrelationParser: process()");
        let message = {
            //let tags = msg.tags();
            let values = msg.values();
            debug!("values: {:?}", &values);
            if let Some(uuid) = values.get(CLASSIFIER_UUID) {
                let name = match values.get(CLASSIFIER_CLASS) {
                    Some(name) => Some(name.borrow()),
                    None => None
                };
                MessageBuilder::new(&uuid, message).values(values.clone()).name(name).build()
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

parser_plugin!(CorrelationParserBuilder<LogParser>);
