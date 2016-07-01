#![feature(test)]
extern crate test;
extern crate kvtagger_rs_parser;
extern crate syslog_ng_common;

use test::Bencher;
use syslog_ng_common::{LogMessage, Parser, ParserBuilder};
use kvtagger_rs_parser::KVTaggerBuilder;

use syslog_ng_common::{syslog_ng_global_init, SYSLOG_NG_INITIALIZED, GlobalConfig, LogTemplate};
use syslog_ng_common::mock::MockPipe;

#[bench]
fn bench_parse(b: &mut Bencher) {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });

    let cfg = GlobalConfig::new(0x0308);
    let csv_file_path = "benches/bigcsv.csv";
    let mut pipe = MockPipe::new();
    let mut builder: KVTaggerBuilder<MockPipe> = ParserBuilder::<MockPipe>::new(cfg.clone());
    let selector = LogTemplate::compile(&cfg, "7.209.203.75".as_bytes()).unwrap();
    builder.set_csv_file(csv_file_path);
    builder.set_lookup_key(selector);
    let mut parser = ParserBuilder::<MockPipe>::build(builder).unwrap();
    let mut logmsg = LogMessage::new();
    let input = "<34>Oct 11 22:14:15 100.100.131.85 su: Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.";
    b.iter(|| parser.parse(&mut pipe, &mut logmsg, input));
}

#[bench]
fn bench_parse_with_prefix(b: &mut Bencher) {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });

    let cfg = GlobalConfig::new(0x0308);
    let csv_file_path = "benches/bigcsv.csv";
    let mut pipe = MockPipe::new();
    let mut builder: KVTaggerBuilder<MockPipe> = ParserBuilder::<MockPipe>::new(cfg.clone());
    let selector = LogTemplate::compile(&cfg, "7.209.203.75".as_bytes()).unwrap();
    builder.set_csv_file(csv_file_path);
    builder.set_lookup_key(selector);
    builder.set_prefix("prefix.".to_string());
    let mut parser = ParserBuilder::<MockPipe>::build(builder).unwrap();
    let mut logmsg = LogMessage::new();
    let input = "<34>Oct 11 22:14:15 100.100.131.85 su: Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.";
    b.iter(|| parser.parse(&mut pipe, &mut logmsg, input));
}

#[bench]
fn bench_parse_with_fallback_to_default_selector(b: &mut Bencher) {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });

    let cfg = GlobalConfig::new(0x0308);
    let csv_file_path = "benches/bigcsv.csv";
    let mut pipe = MockPipe::new();
    let mut builder: KVTaggerBuilder<MockPipe> = ParserBuilder::<MockPipe>::new(cfg.clone());
    let selector = LogTemplate::compile(&cfg, "XXXXXXXXXX".as_bytes()).unwrap();
    builder.set_csv_file(csv_file_path);
    builder.set_lookup_key(selector);
    builder.set_default_selector("7.209.203.75".to_string());
    let mut parser = ParserBuilder::<MockPipe>::build(builder).unwrap();
    let mut logmsg = LogMessage::new();
    let input = "<34>Oct 11 22:14:15 100.100.131.85 su: Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.";
    b.iter(|| parser.parse(&mut pipe, &mut logmsg, input));
}
