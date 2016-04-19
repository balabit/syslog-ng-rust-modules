extern crate syslog_ng_common;

use syslog_ng_common::{GlobalConfig, LogMessage, SYSLOG_NG_INITIALIZED, syslog_ng_global_init, LogTemplate, LogTimeZone, Plugin};

fn test_log_message_context_can_be_formatted(cfg: &GlobalConfig) {
    let mut template = LogTemplate::compile(&cfg, r#"$(grep ("${bar}" == "BAR") ${baz})"#).ok().unwrap();
    let mut msg_1 = LogMessage::new();
    let mut msg_2 = LogMessage::new();
    msg_1.insert("foo", "FOO");
    msg_1.insert("bar", "BAR");
    msg_1.insert("baz", "1");
    msg_2.insert("bar", "BAR");
    msg_2.insert("baz", "2");
    let messages = [msg_1, msg_2];
    let formatted_msg = template.format_with_context(&messages, None, LogTimeZone::Local, 0, None);
    assert_eq!(b"1,2", formatted_msg);
}

fn test_empty_log_message_context_can_be_formatted(cfg: &GlobalConfig) {
    let mut template = LogTemplate::compile(&cfg, r#"$(grep ("${bar}" == "BAR") ${baz})"#).ok().unwrap();
    let messages = [];
    let formatted_msg = template.format_with_context(&messages, None, LogTimeZone::Local, 0, None);
    assert_eq!(b"", formatted_msg);
}

fn main() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let mut cfg = GlobalConfig::new(0x0308);
    Plugin::load_module("basicfuncs", &mut cfg);
    test_log_message_context_can_be_formatted(&cfg);
    test_empty_log_message_context_can_be_formatted(&cfg);
}
