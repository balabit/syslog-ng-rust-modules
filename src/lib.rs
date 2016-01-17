#[macro_use]
extern crate log;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate syslog_ng_sys;
extern crate syslog_ng_common;
extern crate correlation;

use correlation::dispatcher::ResponseHandler;
use correlation::Response;
use correlation::correlator::EventHandler;
use correlation::Correlator;
use correlation::config::Context;
use correlation::message::MessageBuilder;
use serde_json::from_str;
use std::borrow::Borrow;
use std::clone::Clone;
use std::io::Read;
use std::io;
use std::fs::File;
use syslog_ng_common::{MessageFormatter, LogMessage};
use syslog_ng_common::proxies::parser::{Parser, ParserBuilder, OptionError};

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

impl EventHandler<Response> for MessageSender {
    fn handle_event(&mut self, event: Response) {
        if let Response::Message(msg) = event {
            error!("got synthetic message");
        }
    }
    fn handler(&self) -> ResponseHandler {
        ResponseHandler::Message
    }
}

#[derive(Clone)]
pub struct CorrelationParserBuilder {
    contexts: Option<Vec<Context>>,
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

    fn load_contexts(&mut self, path: &str) -> Result<Vec<Context>, Error> {
        let mut file = try!(File::open(path));
        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));
        match from_str::<Vec<Context>>(&buffer) {
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
        debug!("Building Rust parser");
        let CorrelationParserBuilder {contexts, formatter} = self;
        let contexts = try!(contexts.ok_or(OptionError::missing_required_option(options::CONTEXTS_FILE)));
        Ok(CorrelationParser {
            contexts: contexts.clone(),
            correlator: Correlator::new(contexts),
            formatter: formatter
        })
    }
}

pub struct CorrelationParser {
    contexts: Vec<Context>,
    correlator: Correlator,
    formatter: MessageFormatter
}

impl Parser for CorrelationParser {
    fn parse(&mut self, msg: &mut LogMessage, message: &str) -> bool {
        debug!("CorrelationParser: process()");
        let message = {
            //let tags = msg.tags();
            let values = msg.values();
            debug!("values: {:?}", &values);
            let uuid = values.get(".classifier.uuid").expect("Message doesn't have a required '.classifier.uuid' key");
            let name = match values.get(".classifier.class") {
                Some(name) => Some(name.borrow()),
                None => None
            };
            MessageBuilder::new(&uuid, message).values(values.clone()).name(name).build()
        };
        match self.correlator.push_message(message) {
            Ok(_) => true,
            Err(err) => {
                error!("{}", err);
                false
            }
        }
    }
}

impl Clone for CorrelationParser {
    fn clone(&self) -> CorrelationParser {
        CorrelationParser {
            formatter: self.formatter.clone(),
            contexts: self.contexts.clone(),
            correlator: Correlator::new(self.contexts.clone())
        }
    }
}
