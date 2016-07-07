#![feature(test)]
extern crate test;
extern crate kvtagger_rs_parser;
extern crate syslog_ng_common;

use test::Bencher;
use syslog_ng_common::{LogMessage, Parser, ParserBuilder};
use kvtagger_rs_parser::{options, KVTaggerBuilder};
use kvtagger_rs_parser::utils::build_parser;

use syslog_ng_common::{syslog_ng_global_init, SYSLOG_NG_INITIALIZED, GlobalConfig};
use syslog_ng_common::mock::MockPipe;

#[bench]
fn bench_parse(b: &mut Bencher) {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe {
            syslog_ng_global_init();
        }
    });

    let cfg = GlobalConfig::new(0x0308);
    let mut pipe = MockPipe::new();
    let options = [(options::DATABASE, "benches/bigcsv.csv"), (options::SELECTOR, "7.209.203.75")];

    let mut parser = build_parser::<MockPipe, KVTaggerBuilder<_>>(cfg, &options).build().unwrap();
    let mut logmsg = LogMessage::new();
    let input = "<34>Oct 11 22:14:15 100.100.131.85 su: Lorem Ipsum is simply dummy text of the \
                 printing and typesetting industry. Lorem Ipsum has been the industry's standard \
                 dummy text ever since the 1500s, when an unknown printer took a galley of type \
                 and scrambled it to make a type specimen book.";
    b.iter(|| parser.parse(&mut pipe, &mut logmsg, input));
}

#[bench]
fn bench_parse_with_prefix(b: &mut Bencher) {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe {
            syslog_ng_global_init();
        }
    });

    let cfg = GlobalConfig::new(0x0308);
    let mut pipe = MockPipe::new();

    let options = [(options::DATABASE, "benches/bigcsv.csv"),
                   (options::SELECTOR, "7.209.203.75"),
                   (options::PREFIX, "prefix.")];

    let mut parser = build_parser::<MockPipe, KVTaggerBuilder<_>>(cfg, &options).build().unwrap();
    let mut logmsg = LogMessage::new();
    let input = "<34>Oct 11 22:14:15 100.100.131.85 su: Lorem Ipsum is simply dummy text of the \
                 printing and typesetting industry. Lorem Ipsum has been the industry's standard \
                 dummy text ever since the 1500s, when an unknown printer took a galley of type \
                 and scrambled it to make a type specimen book.";
    b.iter(|| parser.parse(&mut pipe, &mut logmsg, input));
}

#[bench]
fn bench_parse_with_fallback_to_default_selector(b: &mut Bencher) {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe {
            syslog_ng_global_init();
        }
    });

    let cfg = GlobalConfig::new(0x0308);
    let mut pipe = MockPipe::new();

    let options = [(options::DATABASE, "benches/bigcsv.csv"),
                   (options::SELECTOR, "XXXXXX"),
                   (options::DEFAULT_SELECTOR, "7.209.203.75")];

    let mut parser = build_parser::<MockPipe, KVTaggerBuilder<_>>(cfg, &options).build().unwrap();

    let mut logmsg = LogMessage::new();
    let input = "<34>Oct 11 22:14:15 100.100.131.85 su: Lorem Ipsum is simply dummy text of the \
                 printing and typesetting industry. Lorem Ipsum has been the industry's standard \
                 dummy text ever since the 1500s, when an unknown printer took a galley of type \
                 and scrambled it to make a type specimen book.";
    b.iter(|| parser.parse(&mut pipe, &mut logmsg, input));
}
