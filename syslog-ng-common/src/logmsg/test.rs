use std::collections::BTreeMap;
use syslog_ng_sys::logmsg::log_msg_registry_init;

use super::LogMessage;

#[test]
fn test_given_empty_log_msg_when_values_are_inserted_then_we_can_get_them_back() {
    unsafe { log_msg_registry_init() };
    let mut logmsg = LogMessage::new();
    let expected_values = {
        let mut values = BTreeMap::new();
        values.insert("foo".to_string(), "bar".to_string());
        values.insert("qux".to_string(), "baz".to_string());
        values
    };

    logmsg.set_value("foo", "bar");
    logmsg.set_value("qux", "baz");
    assert_eq!(&expected_values, &logmsg.values());
}
