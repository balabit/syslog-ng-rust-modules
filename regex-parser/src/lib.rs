#[macro_use]
extern crate syslog_ng_common;
#[macro_use]
extern crate log;
extern crate regex;

use std::marker::PhantomData;
use syslog_ng_common::{LogMessage, Parser, ParserBuilder, OptionError, Pipe, GlobalConfig};
use regex::Regex;

// Example: "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD"
pub const LOGGEN_EXPR: &'static str = r"seq: (?P<seq>\d+), thread: (?P<thread>\d+), runid: (?P<runid>\d+), stamp: (?P<stamp>[^ ]+) (?P<padding>.*$)";
pub const REGEX_OPTION: &'static str = "regex";

#[cfg(test)]
mod tests;

pub struct RegexParser {
    pub regex: Regex,
}

pub struct RegexParserBuilder<P: Pipe> {
    regex: Option<Regex>,
    _marker: PhantomData<P>
}

impl<P> Clone for RegexParserBuilder<P> where P: Pipe {
    fn clone(&self) -> Self {
        RegexParserBuilder {
            regex: self.regex.clone(),
            _marker: PhantomData
        }
    }
}

impl<P: Pipe> ParserBuilder<P> for RegexParserBuilder<P> {
    type Parser = RegexParser;
    fn new(_: GlobalConfig) -> Self {
        RegexParserBuilder { regex: None, _marker: PhantomData }
    }
    fn option(&mut self, name: String, value: String) {
        if name == REGEX_OPTION {
            debug!("Trying to compile regular expression: '{}'", &value);
            match Regex::new(&value) {
                Ok(regex) => self.regex = Some(regex),
                Err(err) => error!("{}", err)
            }
        }
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        debug!("Building Regex parser");
        if let Some(regex) = self.regex {
            Ok(RegexParser { regex: regex })
        } else {
            Err(OptionError::missing_required_option(REGEX_OPTION))
        }
    }
}

impl<P: Pipe> Parser<P> for RegexParser {
    fn parse(&mut self, _: &mut P, logmsg: &mut LogMessage, input: &str) -> bool {
        if let Some(captures) = self.regex.captures(input) {
            for (name, value) in captures.iter_named() {
                if let Some(value) = value {
                    logmsg.insert(name, value.as_bytes());
                }
            }
            true
        } else {
            false
        }
    }
}

parser_plugin!(RegexParserBuilder<LogParser>);
