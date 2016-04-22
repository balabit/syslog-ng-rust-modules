use regex::Regex;
use super::*;

use syslog_ng_common::sys::logmsg::log_msg_registry_init;
use syslog_ng_common::{LogMessage, Parser, ParserBuilder, OptionError, mock, GlobalConfig};

#[test]
fn test_loggen_regex_can_be_compiled() {
    let _ = Regex::new(LOGGEN_EXPR).unwrap();
}

#[test]
fn test_syslog_regex_accepts_valid_syslog_message() {
    let re = Regex::new(LOGGEN_EXPR).unwrap();
    assert_eq!(true,
               re.is_match("seq: 0000000000, thread: 0000, runid: 1456947132, stamp: \
                            2016-03-02T20:32:12 PAD"));
}

#[test]
fn test_syslog_regex_parses_syslog_message() {
    let re = Regex::new(LOGGEN_EXPR).unwrap();
    let caps = re.captures("seq: 0000000000, thread: 0000, runid: 1456947132, stamp: \
                            2016-03-02T20:32:12 PAD")
                 .unwrap();
    assert_eq!("0000000000", caps.name("seq").unwrap());
    assert_eq!("0000", caps.name("thread").unwrap());
    assert_eq!("1456947132", caps.name("runid").unwrap());
    assert_eq!("2016-03-02T20:32:12", caps.name("stamp").unwrap());
    assert_eq!("PAD", caps.name("padding").unwrap());
}

#[test]
fn test_parse_inserts_parsed_named_captures_into_the_logmsg() {
    unsafe {
        log_msg_registry_init();
    };

    let loggen_regex = Regex::new(LOGGEN_EXPR).unwrap();
    let mut parser = RegexParser { regex: loggen_regex };
    let mut logmsg = LogMessage::new();
    let mut pipe = mock::MockPipe::new();
    let input = "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 \
                 PAD";
    parser.parse(&mut pipe, &mut logmsg, input);
    assert_eq!(b"0000000000", logmsg.get("seq").unwrap());
    assert_eq!(b"0000", logmsg.get("thread").unwrap());
    assert_eq!(b"1456947132", logmsg.get("runid").unwrap());
    assert_eq!(b"2016-03-02T20:32:12", logmsg.get("stamp").unwrap());
    assert_eq!(b"PAD", logmsg.get("padding").unwrap());
}

#[test]
fn test_parser_can_be_built_with_valid_regex() {
    let cfg = GlobalConfig::new(0x0308);
    let mut builder = RegexParserBuilder::<mock::MockPipe>::new(cfg);
    builder.option(REGEX_OPTION.to_string(), "[abc]d".to_string());
    let _ = builder.build().unwrap();
}

#[test]
fn test_parser_cannot_be_built_with_invalid_regex() {
    let cfg = GlobalConfig::new(0x0308);
    let mut builder = RegexParserBuilder::<mock::MockPipe>::new(cfg);
    builder.option(REGEX_OPTION.to_string(), "[abcd".to_string());
    assert_eq!(Some(OptionError::missing_required_option(REGEX_OPTION)), builder.build().err());
}
