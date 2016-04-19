use super::*;
use GlobalConfig;
use LogMessage;

use SYSLOG_NG_INITIALIZED;
use syslog_ng_global_init;

#[test]
fn test_template_can_be_created() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let cfg = GlobalConfig::new(0x0308);
    let _ = LogTemplate::new(&cfg);
}

#[test]
fn test_template_can_be_compiled() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let cfg = GlobalConfig::new(0x0308);
    let _ = LogTemplate::compile(&cfg, "literal").ok().unwrap();
}

#[test]
fn test_invalid_template_cannot_be_compiled() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let cfg = GlobalConfig::new(0x0308);
    let _ = LogTemplate::compile(&cfg, "${unbalanced").err().unwrap();
}

#[test]
fn test_log_message_can_be_formatted() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let cfg = GlobalConfig::new(0x0308);
    let mut template = LogTemplate::compile(&cfg, "${kittens}").ok().unwrap();
    let mut msg = LogMessage::new();
    msg.insert("kittens", "2");
    let formatted_msg = template.format(&msg, None, LogTimeZone::Local, 0, None);
    assert_eq!("2", formatted_msg);
}

#[test]
fn test_context_id_can_be_used() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let cfg = GlobalConfig::new(0x0308);
    let mut template = LogTemplate::compile(&cfg, "${CONTEXT_ID}").ok().unwrap();
    let msg = LogMessage::new();
    let actual = template.format(&msg, None, LogTimeZone::Local, 0, Some("context-id"));
    assert_eq!("context-id", actual);
}
