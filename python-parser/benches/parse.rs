#![feature(test)]
extern crate test;
extern crate python_parser;
extern crate syslog_ng_common;
extern crate env_logger;

use std::env;
use test::Bencher;
use syslog_ng_common::{LogMessage, Parser};
use python_parser::utils::build_parser_with_options;

use syslog_ng_common::{SYSLOG_NG_INITIALIZED, syslog_ng_global_init};
use syslog_ng_common::mock::MockPipe;

#[bench]
fn bench_parse(b: &mut Bencher) {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let mut pipe = MockPipe::new();
    let options = [("regex", r#"seq: (?P<seq>\d+), thread: (?P<thread>\d+), runid: (?P<runid>\d+), stamp: (?P<stamp>[^ ]+) (?P<padding>.*$)"#)];
    let message = "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD";
    let mut parser = build_parser_with_options("_test_module.regex", "RegexParser", &options);
    let mut logmsg = LogMessage::new();
    b.iter(|| parser.parse(&mut pipe, &mut logmsg, message));
}
