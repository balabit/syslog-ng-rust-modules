#[macro_use]
extern crate log;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate syslog_ng_sys;
extern crate syslog_ng_common;
extern crate correlation;

use correlation::Correlator;
use correlation::config::Context;
use serde_json::from_str;
use std::borrow::Borrow;
use std::clone::Clone;
use std::io::{
    Read
};
use std::io;
use std::fs::File;
use syslog_ng_common::MessageFormatter;
use syslog_ng_sys::{
    RustParser,
    LogMessage
};

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

pub struct CorrelationParser {
    correlator: Option<Correlator>,
    config_contexts: Option<Vec<Context>>,
    formatter: MessageFormatter
}

impl CorrelationParser {
    pub fn new() -> CorrelationParser {
        debug!("CorrelationParser: new()");
        CorrelationParser {
            correlator: None,
            config_contexts: None,
            formatter: MessageFormatter::new()
        }
    }

    pub fn set_file(&mut self, path: &str) {
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

impl RustParser for CorrelationParser {
    fn process(&mut self, msg: &mut LogMessage, input: &str) -> bool {
        false
    }

    fn init(&mut self) -> bool {
        debug!("CorrelationParser: init()");
        if self.correlator.is_none() {
            error!("CorrelationParser: not all required parameters are set");
            false
        } else {
            true
        }
    }

    fn set_option(&mut self, key: String, value: String) {
        debug!("CorrelationParser: set_option(key={}, value={})", &key, &value);

        match key.borrow() {
            "contexts_file" => {
                self.set_file(&value);
            },
            "prefix" => {
                self.set_prefix(value);
            },
            _ => {
                debug!("CorrelationParser: not supported key: {:?}", key) ;
            }
        };
    }

    fn boxed_clone(&self) -> Box<RustParser> {
        Box::new(self.clone())
    }
}

impl Clone for CorrelationParser {
    fn clone(&self) -> CorrelationParser {
        let contexts: Option<Vec<Context>> = match self.config_contexts {
            Some(ref contexts) => Some(contexts.clone()),
            None => None
        };
        CorrelationParser {
            correlator: None,
            config_contexts: contexts,
            formatter: self.formatter.clone()
        }
    }
}
