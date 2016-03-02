#![feature(test)]
extern crate test;
extern crate regex;
extern crate regex_parser;
extern crate syslog_ng_common;

use test::Bencher;
use regex::Regex;
use syslog_ng_common::{LogMessage, Parser};
use regex_parser::{RegexParser, LOGGEN_EXPR};

use syslog_ng_common::sys::logmsg::log_msg_registry_init;

#[bench]
fn bench_parse(b: &mut Bencher) {
    unsafe {
        // We have to initialize some global variables in syslog-ng
        log_msg_registry_init();
    };

    let loggen_regex = Regex::new(LOGGEN_EXPR).unwrap();
    let mut parser = RegexParser {regex: loggen_regex};
    let mut logmsg = LogMessage::new();
    let input = "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD";
    b.iter(|| parser.parse(&mut logmsg, input));
}
