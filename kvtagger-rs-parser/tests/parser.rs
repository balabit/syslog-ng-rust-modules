extern crate kvtagger_rs_parser;
extern crate syslog_ng_common;

use kvtagger_rs_parser::{KVTagger, LookupTable, KvTaggerBuilder};
use kvtagger_rs_parser::utils::{make_expected_value_for_test_file};

use syslog_ng_common::{LogMessage, Parser, SYSLOG_NG_INITIALIZED, syslog_ng_global_init, ParserBuilder, GlobalConfig};
use syslog_ng_common::mock::MockPipe;

#[test]
fn test_parser_enriches_the_message_with_key_value_pairs() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let records = make_expected_value_for_test_file();

    let mut parser = KVTagger {
        map: LookupTable::new(records),
        lookup_key: "key3".to_string()
    };

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

    let mut builder = KvTaggerBuilder::<MockPipe>::new(cfg);
    builder.option("csv-file".to_string(), "tests/test.csv".to_string());
    builder.option("lookup-key".to_string(), "key3".to_string());

    let mut parser = builder.build().unwrap();

    let mut logmsg = LogMessage::new();
    let mut mock_pipe = MockPipe::new();

    parser.parse(&mut mock_pipe, &mut logmsg, "message");

    assert_eq!(logmsg.get("name18").unwrap(), b"value18");
    assert_eq!(logmsg.get("name19").unwrap(), b"value19");
    assert_eq!(logmsg.get("name20").unwrap(), b"value20");
    assert_eq!(logmsg.get("non_existing_name"), None);
}

#[test]
fn test_parser_cannot_be_built_without_lookup_key() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);

    let mut builder = KvTaggerBuilder::<MockPipe>::new(cfg);
    builder.option("csv-file".to_string(), "tests/test.csv".to_string());

    let _ = builder.build().err().unwrap();
}

#[test]
fn test_parser_cannot_be_built_without_csv_file() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);

    let mut builder = KvTaggerBuilder::<MockPipe>::new(cfg);
    builder.option("lookup-key".to_string(), "key3".to_string());

    let _ = builder.build().err().unwrap();
}

#[test]
fn test_parser_cannot_be_built_without_configuration_options() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init() };
    });

    let cfg = GlobalConfig::new(0x0308);
    let builder = KvTaggerBuilder::<MockPipe>::new(cfg);
    let _ = builder.build().err().unwrap();
}
