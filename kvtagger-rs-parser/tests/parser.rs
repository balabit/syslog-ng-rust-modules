extern crate kvtagger_rs_parser;
extern crate syslog_ng_common;

use kvtagger_rs_parser::{KVTaggerBuilder, options};
use kvtagger_rs_parser::utils::build_parser;

use syslog_ng_common::{LogMessage, Parser, SYSLOG_NG_INITIALIZED, syslog_ng_global_init, ParserBuilder, GlobalConfig};
use syslog_ng_common::mock::MockPipe;

#[test]
fn test_parser_enriches_the_message_with_key_value_pairs() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);
    let options = [
        (options::SELECTOR, "key3"),
        (options::DATABASE, "tests/test.csv"),
    ];

    let mut parser = build_parser::<MockPipe, KVTaggerBuilder<_>>(cfg, &options).build().unwrap();

    let mut logmsg = LogMessage::new();
    let mut mock_pipe = MockPipe::new();

    parser.parse(&mut mock_pipe, &mut logmsg, "message");

    assert_eq!(logmsg.get("name18").unwrap(), b"value18");
    assert_eq!(logmsg.get("name19").unwrap(), b"value19");
    assert_eq!(logmsg.get("name20").unwrap(), b"value20");
    assert_eq!(logmsg.get("non_existing_name"), None);
}

#[test]
fn test_kv_tagger_can_be_constructed_from_options() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);
    let options = [
        (options::SELECTOR, "key3"),
        (options::DATABASE, "tests/test.csv"),
    ];

    let mut parser = build_parser::<MockPipe, KVTaggerBuilder<_>>(cfg, &options).build().unwrap();

    let mut logmsg = LogMessage::new();
    let mut mock_pipe = MockPipe::new();

    parser.parse(&mut mock_pipe, &mut logmsg, "message");

    assert_eq!(logmsg.get("name18").unwrap(), b"value18");
    assert_eq!(logmsg.get("name19").unwrap(), b"value19");
    assert_eq!(logmsg.get("name20").unwrap(), b"value20");
    assert_eq!(logmsg.get("non_existing_name"), None);
}

#[test]
fn test_parser_cannot_be_built_without_selector() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);

    let options = [
        (options::DATABASE, "tests/test.csv"),
    ];

    let _ = build_parser::<MockPipe, KVTaggerBuilder<_>>(cfg, &options).build().err().unwrap();
}

#[test]
fn test_parser_cannot_be_built_without_csv_file() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);
    let options = [
        (options::SELECTOR, "key3"),
    ];

    let _ = build_parser::<MockPipe, KVTaggerBuilder<_>>(cfg, &options).build().err().unwrap();
}

#[test]
fn test_parser_cannot_be_built_without_configuration_options() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);
    let builder = KVTaggerBuilder::<MockPipe>::new(cfg);
    let _ = builder.build().err().unwrap();
}

#[test]
fn test_parser_can_use_templates_as_selector() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);

    let options = [
        (options::SELECTOR, "${PART_1}${PART_2}"),
        (options::DATABASE, "tests/test.csv"),
    ];

    let mut parser = build_parser::<MockPipe, KVTaggerBuilder<_>>(cfg, &options).build().unwrap();

    let mut logmsg = LogMessage::new();
    logmsg.insert("PART_1", "key".as_bytes());
    logmsg.insert("PART_2", "3".as_bytes());

    let mut mock_pipe = MockPipe::new();

    parser.parse(&mut mock_pipe, &mut logmsg, "message");

    assert_eq!(logmsg.get("name18").unwrap(), b"value18");
    assert_eq!(logmsg.get("name19").unwrap(), b"value19");
    assert_eq!(logmsg.get("name20").unwrap(), b"value20");
}

#[test]
fn test_prefix_can_be_set_on_parser() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);
    let options = [
        (options::SELECTOR, "key3"),
        (options::PREFIX, "prefix."),
        (options::DATABASE, "tests/test.csv"),
    ];
    let mut parser = build_parser::<MockPipe, KVTaggerBuilder<_>>(cfg, &options).build().unwrap();
    let mut logmsg = LogMessage::new();
    let mut mock_pipe = MockPipe::new();

    parser.parse(&mut mock_pipe, &mut logmsg, "message");

    assert_eq!(logmsg.get("prefix.name18").unwrap(), b"value18");
    assert_eq!(logmsg.get("prefix.name19").unwrap(), b"value19");
    assert_eq!(logmsg.get("prefix.name20").unwrap(), b"value20");
}

#[test]
fn test_parser_uses_default_selector() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);

    let options = [
        (options::DATABASE, "tests/test.csv"),
        (options::DEFAULT_SELECTOR, "key3"),
        (options::SELECTOR, "XXXX"),
    ];
    let mut parser = build_parser::<MockPipe, KVTaggerBuilder<MockPipe>>(cfg, &options).build().unwrap();

    let mut logmsg = LogMessage::new();
    let mut mock_pipe = MockPipe::new();

    parser.parse(&mut mock_pipe, &mut logmsg, "message");

    assert_eq!(logmsg.get("name18").unwrap(), b"value18");
    assert_eq!(logmsg.get("name19").unwrap(), b"value19");
    assert_eq!(logmsg.get("name20").unwrap(), b"value20");
}
