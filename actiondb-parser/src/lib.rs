#[macro_use]
extern crate log;

#[macro_use]
extern crate syslog_ng_sys;
extern crate actiondb;

use std::borrow::Borrow;
use std::clone;
use std::fmt::Write;

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
        let mut prefixed_key = String::new();
        for &(key, value) in result.pairs() {
            self.set_value_in_logmsg(msg, &mut prefixed_key, key, value);
        }

        if let Some(name) = result.pattern().name() {
            self.set_value_in_logmsg(msg, &mut prefixed_key, keys::PATTERN_NAME, name);
        }

        let uuid = result.pattern().uuid().to_hyphenated_string();
        self.set_value_in_logmsg(msg, &mut prefixed_key, keys::PATTERN_UUID, &uuid);
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = Some(prefix);
    }

    fn prepend_prefix(&self, key: &str, buffer: &mut String) {
        match self.prefix.as_ref() {
            Some(prefix) => {
                let _ = buffer.write_str(prefix);
                let _ = buffer.write_str(key);
            },
            None => {
                let _ = buffer.write_str(key);
            }
        };
    }

    fn set_value_in_logmsg(&self, msg: &mut LogMessage, buffer: &mut String, key: &str, value: &str) {
        self.prepend_prefix(key, buffer);
        msg.set_value(&buffer, value);
        buffer.clear();
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
