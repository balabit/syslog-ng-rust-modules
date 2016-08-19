#[macro_use]
extern crate syslog_ng_common;
#[macro_use]
extern crate log;
extern crate regex;

use syslog_ng_common::{LogMessage, Parser, ParserBuilder, Error, Pipe, GlobalConfig};
use regex::Regex;

// Example: "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD"
pub const LOGGEN_EXPR: &'static str = r"seq: (?P<seq>\d+), thread: (?P<thread>\d+), runid: (?P<runid>\d+), stamp: (?P<stamp>[^ ]+) (?P<padding>.*$)";
pub const REGEX_OPTION: &'static str = "regex";

#[cfg(test)]
mod tests;

pub struct RegexParser {
    pub regex: Regex,
}

pub struct RegexParserBuilder {
    regex: Option<Regex>,
}

impl Clone for RegexParserBuilder {
    fn clone(&self) -> Self {
        RegexParserBuilder {
            regex: self.regex.clone(),
        }
    }
}

impl ParserBuilder for RegexParserBuilder {
    type Parser = RegexParser;
    fn new(_: GlobalConfig) -> Self {
        RegexParserBuilder { regex: None }
    }
    fn option(&mut self, name: String, value: String) -> Result<(), Error> {
        if name == REGEX_OPTION {
            debug!("Trying to compile regular expression: '{}'", &value);
            match Regex::new(&value) {
                Ok(regex) => {
                    self.regex = Some(regex);
                    Ok(())
                },
                Err(err) => Err(Error::verbatim_error(format!("{}", err)))
            }
        } else {
            Err(Error::unknown_option(name))
        }
    }
    fn build(self) -> Result<Self::Parser, Error> {
        debug!("Building Regex parser");
        let regex = try!(self.regex.ok_or(Error::missing_required_option(REGEX_OPTION)));
        Ok(RegexParser { regex: regex })
    }
}

impl Parser for RegexParser {
    fn parse(&mut self, _: &mut Pipe, logmsg: &mut LogMessage, input: &str) -> bool {
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

parser_plugin!(RegexParserBuilder);
