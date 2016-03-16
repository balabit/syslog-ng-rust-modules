#[macro_use]
extern crate log;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate syslog_ng_common;
extern crate correlation;

use correlation::{Request, ContextMap, MessageBuilder, Alert};
use correlation::config::action::message::InjectMode;
use correlation::correlator::{Correlator, AlertHandler};
use correlation::config::ContextConfig;
use serde_json::from_str;
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::io::{self, Read};
use std::fs::File;
use std::sync::{mpsc, Arc, Mutex};
use syslog_ng_common::{MessageFormatter, LogMessage};
use syslog_ng_common::{Parser, ParserBuilder, OptionError, Pipe};

pub mod options;

pub const CLASSIFIER_UUID: &'static str = ".classifier.uuid";
pub const CLASSIFIER_CLASS: &'static str = ".classifier.class";

#[derive(Debug)]
enum Error {
    Io(io::Error),
    SerdeJson(serde_json::error::Error)
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    }
}

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
        match self.load_contexts(path) {
            Ok(contexts) => {
                self.contexts = Some(contexts);
            },
            Err(err) => {
                error!("CorrelationParser: failed to set config file: {:?}", &err);
            }
        }
    }

    fn load_contexts(&mut self, path: &str) -> Result<Vec<ContextConfig>, Error> {
        let mut file = try!(File::open(path));
        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));
        match from_str::<Vec<ContextConfig>>(&buffer) {
            Ok(contexts) => Ok(contexts),
            Err(error) => {
                error!("CorrelationParser: failed to load correlation contexts from file: {}", &error);
                Err(Error::from(error))
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
