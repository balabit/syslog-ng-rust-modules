use std::collections::BTreeMap;
use std::mem;

use super::LogMessage;

#[test]
fn test_given_empty_log_msg_when_values_are_inserted_then_we_can_get_them_back() {
    unsafe {
        let logmsg = LogMessage::new();
        let logmsg = mem::transmute::<*mut LogMessage, &mut LogMessage>(logmsg);
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
}
