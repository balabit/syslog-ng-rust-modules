// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::BTreeMap;

use super::LogMessage;
use SYSLOG_NG_INITIALIZED;
use syslog_ng_global_init;

#[test]
fn test_given_empty_log_msg_when_values_are_inserted_then_we_can_get_them_back() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let mut logmsg = LogMessage::new();
    let expected_values = {
        let mut values = BTreeMap::new();
        values.insert("foo".to_string(), "bar".to_string());
        values.insert("qux".to_string(), "baz".to_string());
        values
    };

    logmsg.insert("foo", "bar");
    logmsg.insert("qux", "baz");
    assert_eq!(&expected_values, &logmsg.values());
}

#[test]
fn test_given_empty_log_msg_when_a_not_inserted_key_is_looked_up_then_get_returns_none() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let mut logmsg = LogMessage::new();
    logmsg.insert("foo", "bar");
    assert_eq!(None, logmsg.get("ham"));
}

#[test]
fn test_log_msg_get_returns_the_expected_value() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let mut logmsg = LogMessage::new();
    logmsg.insert("foo", "bar");
    let expected = b"bar";
    let actual = logmsg.get("foo");
    assert_eq!(Some(&expected[..]), actual);
}
