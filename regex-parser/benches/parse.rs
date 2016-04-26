#![feature(test)]
extern crate test;
extern crate regex;
extern crate regex_parser;
extern crate syslog_ng_common;

use test::Bencher;
use regex::Regex;
use syslog_ng_common::{LogMessage, Parser, mock, SYSLOG_NG_INITIALIZED, syslog_ng_global_init};
use regex_parser::{RegexParser, LOGGEN_EXPR};

#[bench]
fn bench_parse(b: &mut Bencher) {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });

    let loggen_regex = Regex::new(LOGGEN_EXPR).unwrap();
    let mut parser = RegexParser { regex: loggen_regex };
    let mut logmsg = LogMessage::new();
    let mut pipe = mock::MockPipe::new();
    let input = "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD";
    b.iter(|| parser.parse(&mut pipe, &mut logmsg, input));
}
