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
    formatter: MessageFormatter
}

impl ActiondbParser {
    pub fn new() -> ActiondbParser {
        debug!("ActiondbParser: new()");
        ActiondbParser{
            matcher: None,
            formatter: MessageFormatter::new()
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

    pub fn set_prefix(&mut self, prefix: String) {
        self.formatter.set_prefix(prefix);
    }
}

impl RustParser for ActiondbParser {
    fn process(&mut self, msg: &mut LogMessage, input: &str) -> bool {
        if let Some(result) = self.matcher.as_ref().unwrap().parse(input) {
            MessageFiller::fill_logmsg(&mut self.formatter, msg, &result);
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
                    formatter: self.formatter.clone(),
                }
            },
            Option::None => {
                ActiondbParser{
                    matcher: None,
                    formatter: self.formatter.clone(),
                }
            }
        }
    }
}

struct MessageFiller;

impl MessageFiller {
    fn fill_logmsg(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        MessageFiller::fill_values(formatter, msg, result);
        MessageFiller::fill_name(formatter, msg, result);
        MessageFiller::fill_uuid(formatter, msg, result);
        MessageFiller::fill_tags(msg, result);
    }

    fn fill_values(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        MessageFiller::fill_parsed_values(formatter, msg, result);
        MessageFiller::fill_additional_values(formatter, msg, result);
    }

    fn fill_parsed_values(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        for &(key, value) in result.pairs() {
            let (key, value) = formatter.format(key, value);
            msg.set_value(key, value);
        }
    }

    fn fill_additional_values(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        if let Some(values) = result.pattern().values() {
            for (key, value) in values {
                let (key, value) = formatter.format(key, value);
                msg.set_value(key, value);
            }
        }
    }

    fn fill_name(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        if let Some(name) = result.pattern().name() {
            let (key, value) = formatter.format(keys::PATTERN_NAME, name);
            msg.set_value(key, value);
        }
    }

    fn fill_uuid(formatter: &mut MessageFormatter, msg: &mut LogMessage, result: &MatchResult) {
        let uuid = result.pattern().uuid().to_hyphenated_string();
        let (key, value) = formatter.format(keys::PATTERN_UUID, &uuid);
        msg.set_value(key, value);
    }

    fn fill_tags(msg: &mut LogMessage, result: &MatchResult) {
        if let Some(tags) = result.pattern().tags() {
            for i in tags {
                msg.set_tag(i);
            }
        }
    }
}

#[derive(Clone)]
struct MessageFormatter {
    buffer: String,
    prefix: Option<String>
}

impl MessageFormatter {
    fn new() -> MessageFormatter {
        MessageFormatter {
            buffer: String::new(),
            prefix: None
        }
    }

    fn set_prefix(&mut self, prefix: String) {
        self.prefix = Some(prefix)
    }

    fn format<'a, 'b, 'c>(&'a mut self, key: &'b str, value: &'c str) -> (&'a str, &'c str) {
        self.buffer.clear();
        self.apply_prefix(key);
        (&self.buffer, value)
    }

    fn apply_prefix(&mut self, key: &str) {
        match self.prefix.as_ref() {
            Some(prefix) => {
                let _ = self.buffer.write_str(prefix);
                let _ = self.buffer.write_str(key);
            },
            None => {
                let _ = self.buffer.write_str(key);
            }
        };
    }
}
