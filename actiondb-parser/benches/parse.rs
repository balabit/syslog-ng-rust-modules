#![feature(test)]
extern crate test;
extern crate actiondb_parser;
extern crate actiondb;
extern crate syslog_ng_common;

use test::Bencher;
use syslog_ng_common::{LogMessage, Parser, ParserBuilder};

use syslog_ng_common::{syslog_ng_global_init, SYSLOG_NG_INITIALIZED, GlobalConfig};
use syslog_ng_common::mock::MockPipe;
use actiondb_parser::ActiondbParserBuilder;
use actiondb::matcher::suffix_array::{SuffixArrayMatcherSuite};

#[bench]
fn bench_parse(b: &mut Bencher) {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });

    let cfg = GlobalConfig::new(0x0308);
    let pattern_file_path = "benches/loggen.json";
    let mut pipe = MockPipe::new();
    let mut builder: ActiondbParserBuilder<SuffixArrayMatcherSuite> = ParserBuilder::new(cfg);
    builder.set_pattern_file(pattern_file_path);
    let mut parser = ParserBuilder::build(builder).unwrap();
    let mut logmsg = LogMessage::new();
    let input = "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD";
    b.iter(|| parser.parse(&mut pipe, &mut logmsg, input));
}
