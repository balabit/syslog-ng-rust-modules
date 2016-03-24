#![feature(test)]
extern crate test;
extern crate python_parser;
extern crate syslog_ng_common;
extern crate env_logger;

use std::env;
use test::Bencher;
use syslog_ng_common::{LogMessage, Parser};
use python_parser::utils::build_parser_with_options;

use syslog_ng_common::sys::logmsg::log_msg_registry_init;

#[bench]
fn bench_parse(b: &mut Bencher) {
    unsafe {
        log_msg_registry_init();
    };
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let options = [("regex", r#"seq: (?P<seq>\d+), thread: (?P<thread>\d+), runid: (?P<runid>\d+), stamp: (?P<stamp>[^ ]+) (?P<padding>.*$)"#)];
    let message = "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD";
    let mut parser = build_parser_with_options("_test_module.regex", "RegexParser", &options);
    let mut logmsg = LogMessage::new();
    b.iter(|| parser.parse(&mut logmsg, message));
}
