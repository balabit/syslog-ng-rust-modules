#[macro_use]
extern crate log;

#[macro_use]
extern crate syslog_ng_sys;
extern crate actiondb;

use std::borrow::Borrow;
use std::clone;

use actiondb::matcher::Matcher;
use actiondb::matcher::Factory;
use actiondb::matcher::result::MatchResult;

use syslog_ng_sys::{RustParser,
                    LogMessage};

mod keys {
    pub const PATTERN_NAME: &'static str = ".classifier.class";
    pub const PATTERN_UUID: &'static str = ".classifier.uuid";
}

pub struct ActiondbParser {
    matcher: Option<Box<Matcher>>,
    prefix: Option<String>,
}

impl ActiondbParser {
    pub fn new() -> ActiondbParser {
        debug!("ActiondbParser: new()");
        ActiondbParser{
            matcher: None,
            prefix: None
        }
    }

    pub fn set_pattern_file(&mut self, path: &str) {
        match Factory::from_file(path) {
            Ok(matcher) => {
                self.matcher = Some(matcher)
            },
            Err(err) => {
                error!("ActiondbParser: failed to set 'pattern_file'");
                error!("{}", err);
            }
        }
    }

    pub fn populate_logmsg(&self, msg: &mut LogMessage, result: &MatchResult) {
        for &(key, value) in result.pairs() {
            msg.set_value(key, value);
        }

        if let Some(name) = result.pattern().name() {
            msg.set_value(keys::PATTERN_NAME, name);
        }

        msg.set_value(keys::PATTERN_UUID, &result.pattern().uuid().to_hyphenated_string());
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = Some(prefix);
    }
}

impl RustParser for ActiondbParser {
    fn process(&self, msg: &mut LogMessage, input: &str) -> bool {
        if let Some(result) = self.matcher.as_ref().unwrap().parse(input) {
            self.populate_logmsg(msg, &result);
            true
        } else {
            false
        }
    }

    fn init(&mut self) -> bool {
        debug!("ActiondbParser: init()");
        if self.matcher.is_none() {
            error!("ActiondbParser: not all required parameters are set");
            false
        } else {
            true
        }
    }

    fn set_option(&mut self, key: String, value: String) {
        debug!("ActiondbParser: set_option(key={}, value={})", &key, &value);

        match key.borrow() {
            "pattern_file" => {
                self.set_pattern_file(&value);
            },
            "prefix" => {
                self.set_prefix(value);
            },
            _ => {
                debug!("ActiondbParser: not supported key: {:?}", key) ;
            }
        };
    }

    fn boxed_clone(&self) -> Box<RustParser> {
        Box::new(self.clone())
    }
}

impl clone::Clone for ActiondbParser {
    fn clone(&self) -> ActiondbParser {
        match self.matcher.as_ref() {
            Option::Some(matcher) => {
                ActiondbParser{
                    matcher: Some(matcher.boxed_clone()),
                    prefix: self.prefix.clone(),
                }
            },
            Option::None => {
                ActiondbParser{
                    matcher: None,
                    prefix: self.prefix.clone(),
                }
            }
        }
    }
}
