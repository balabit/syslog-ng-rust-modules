#[macro_use]
extern crate syslog_ng_common;
#[macro_use]
extern crate log;
extern crate regex;

use syslog_ng_common::{LogMessage, Parser, ParserBuilder, OptionError};
use regex::Regex;

// Example: "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD"
const _LOGGEN_EXPR: &'static str = r"seq: (?P<seq>\d+), thread: (?P<thread>\d+), runid: (?P<runid>\d+), stamp: (?P<stamp>[^ ]+) (?P<padding>.*$)";

#[test]
fn test_loggen_regex_can_be_compiled() {
    let _ = Regex::new(_LOGGEN_EXPR).unwrap();
}

#[test]
fn test_syslog_regex_accepts_valid_syslog_message() {
    let re = Regex::new(_LOGGEN_EXPR).unwrap();
    assert_eq!(true, re.is_match("seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD"));
}

#[test]
fn test_syslog_regex_parses_syslog_message() {
    let re = Regex::new(_LOGGEN_EXPR).unwrap();
    let caps = re.captures("seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD").unwrap();
    assert_eq!("0000000000", caps.name("seq").unwrap());
    assert_eq!("0000", caps.name("thread").unwrap());
    assert_eq!("1456947132", caps.name("runid").unwrap());
    assert_eq!("2016-03-02T20:32:12", caps.name("stamp").unwrap());
    assert_eq!("PAD", caps.name("padding").unwrap());
}

#[derive(Clone)]
pub struct RegexParser {
    regex: Regex
}

pub struct RegexParserBuilder;

impl ParserBuilder for RegexParserBuilder {
    type Parser = RegexParser;
    fn new() -> Self {
        RegexParserBuilder
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        debug!("Building Regex parser");
        Ok(RegexParser {
            regex: Regex::new(LOGGEN_EXPR).unwrap()
        })
    }
}

impl Parser for RegexParser {
    fn parse(&mut self, logmsg: &mut LogMessage, input: &str) -> bool {
        if let Some(captures) = self.regex.captures(input) {
            for (name, value) in captures.iter_named() {
                if let Some(value) = value {
                    logmsg.insert(name, value);
                }
            }
            true
        } else {
            false
        }
    }
}

parser_plugin!(RegexParserBuilder);
