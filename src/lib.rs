#[macro_use]
extern crate log;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate syslog_ng_common;
extern crate correlation;

use correlation::{Response, Request, EventHandler, ContextMap, ResponseHandle, MessageBuilder};
use correlation::config::action::message::InjectMode;
use correlation::correlator::Correlator;
use correlation::config::ContextConfig;
use serde_json::from_str;
use std::borrow::Borrow;
use std::io::{self, Read};
use std::fs::File;
use std::sync::{mpsc, Arc, Mutex};
use syslog_ng_common::{MessageFormatter, LogMessage};
use syslog_ng_common::{Parser, ParserBuilder, OptionError};

pub mod options;

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

impl EventHandler<Response, mpsc::Sender<Request>> for MessageSender {
    fn handle_event(&mut self, event: Response, reactor_input_channel: &mut mpsc::Sender<Request>) {
        if let Response::Alert(msg) = event {
            match msg.inject_mode {
                InjectMode::Log => {
                    debug!("{}", msg.message.message());
                },
                InjectMode::Forward => {},
                InjectMode::Loopback => {
                    if let Err(err) = reactor_input_channel.send(Request::Message(Arc::new(msg.message))) {
                        error!("{}", err);
                    }
                },
            }
        }
    }
    fn handle(&self) -> ResponseHandle {
        ResponseHandle::Alert
    }
}

pub struct CorrelationParserBuilder {
    contexts: Option<Vec<ContextConfig>>,
    formatter: MessageFormatter
}

impl CorrelationParserBuilder {
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

impl ParserBuilder for CorrelationParserBuilder {
    type Parser = CorrelationParser;
    fn new() -> Self {
        CorrelationParserBuilder {
            contexts: None,
            formatter: MessageFormatter::new()
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
        let CorrelationParserBuilder {contexts, formatter} = self;
        let contexts = try!(contexts.ok_or(OptionError::missing_required_option(options::CONTEXTS_FILE)));
        let map = ContextMap::from_configs(contexts);
        let mut correlator = Correlator::new(map);
        correlator.register_handler(Box::new(MessageSender));
        Ok(CorrelationParser::new(correlator, formatter))
    }
}

#[derive(Clone)]
pub struct CorrelationParser {
    correlator: Arc<Mutex<Correlator>>,
    formatter: MessageFormatter
}

impl CorrelationParser {
    pub fn new(correlator: Correlator, formatter: MessageFormatter) -> CorrelationParser {
        CorrelationParser {
            correlator: Arc::new(Mutex::new(correlator)),
            formatter: formatter,
        }
    }
}

impl Parser for CorrelationParser {
    fn parse(&mut self, msg: &mut LogMessage, message: &str) -> bool {
        debug!("CorrelationParser: process()");
        let message = {
            //let tags = msg.tags();
            let values = msg.values();
            debug!("values: {:?}", &values);
            if let Some(uuid) = values.get(".classifier.uuid") {
                let name = match values.get(".classifier.class") {
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

parser_plugin!(CorrelationParserBuilder);
