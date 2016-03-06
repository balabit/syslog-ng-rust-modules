#![feature(test)]
extern crate test;
extern crate actiondb_parser;
extern crate actiondb;
extern crate syslog_ng_common;

use test::Bencher;
use syslog_ng_common::{LogMessage, Parser, ParserBuilder};

use syslog_ng_common::sys::logmsg::log_msg_registry_init;
use actiondb_parser::ActiondbParserBuilder;
use actiondb::matcher::suffix_array::SuffixArrayMatcherSuite;

#[bench]
fn bench_parse(b: &mut Bencher) {
    unsafe {
        // We have to initialize some global variables in syslog-ng
        log_msg_registry_init();
    };

    let pattern_file_path = "benches/loggen.json";
    let mut builder: ActiondbParserBuilder<SuffixArrayMatcherSuite> = ActiondbParserBuilder::new();
    builder.set_pattern_file(pattern_file_path);
    let mut parser = builder.build().unwrap();
    let mut logmsg = LogMessage::new();
    let input = "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD";
    b.iter(|| parser.parse(&mut logmsg, input));
}
