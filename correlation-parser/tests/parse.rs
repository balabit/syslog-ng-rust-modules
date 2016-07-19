extern crate correlation_parser;
extern crate correlation;
extern crate syslog_ng_common;
extern crate env_logger;

use correlation_parser::{CorrelationParserBuilder, options, CLASSIFIER_UUID, CLASSIFIER_CLASS};
use correlation_parser::mock::{MockEvent, MockLogTemplate, MockLogTemplateFactory, MockTimer};
use syslog_ng_common::{ParserBuilder, LogMessage, Parser, SYSLOG_NG_INITIALIZED, syslog_ng_global_init, GlobalConfig};
use syslog_ng_common::mock::MockPipe;

use std::time::Duration;

#[test]
fn test_alert_is_forwarded() {
    let _ = env_logger::init();
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let mut logmsg = LogMessage::new();
    logmsg.insert(CLASSIFIER_UUID, b"9cd7a5d6-d439-484d-95ac-7bf3bd055082");
    logmsg.insert(CLASSIFIER_CLASS, b"LOGGEN");

    let config_file = "tests/contexts.json";
    let message = "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD";

    let mut pipe = MockPipe::new();
    let cfg = GlobalConfig::new(0x0308);
    let mut builder = CorrelationParserBuilder::<MockPipe, MockEvent, MockLogTemplate, MockLogTemplateFactory, MockTimer<MockEvent, MockLogTemplate>>::new(cfg);
    builder.option(options::CONTEXTS_FILE.to_owned(), config_file.to_owned()).ok().unwrap();
    let mut parser = builder.build().unwrap();
    let timer = parser.timer.clone();
    assert_eq!(true, parser.parse(&mut pipe, &mut logmsg, message));
    assert_eq!(true, parser.parse(&mut pipe, &mut logmsg, message));
    assert_eq!(0, pipe.forwarded_messages.len());
    timer.elapse_time(Duration::from_secs(3));
    assert_eq!(0, pipe.forwarded_messages.len());
    timer.elapse_time(Duration::from_secs(2));
    // after 5 secs we should get one message when the parses next gets access to the pipe
    assert_eq!(true, parser.parse(&mut pipe, &mut logmsg, message));
    assert_eq!(1, pipe.forwarded_messages.len());
    let alert = pipe.forwarded_messages.get(0).unwrap();
    for i in alert.values() {
        println!("{:?}", i);
    }
    assert_eq!(b"artificial test message", alert.get(&b"MESSAGE"[..]).unwrap());
}

#[test]
fn test_syslog_ng_does_not_spin_with_invalid_yaml_configuration() {
    let _ = env_logger::init();
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });

    let config_file = "tests/spinning.yml";

    let cfg = GlobalConfig::new(0x0308);
    let mut builder = CorrelationParserBuilder::<MockPipe, MockEvent, MockLogTemplate, MockLogTemplateFactory, MockTimer<MockEvent, MockLogTemplate>>::new(cfg);
    builder.option(options::CONTEXTS_FILE.to_owned(), config_file.to_owned()).err().unwrap();
    let _ = builder.build().err().unwrap();
}
