#[macro_use]
extern crate syslog_ng_sys;

use syslog_ng_sys::{RustParser,
                    LogMessage};

pub struct DummyParser;

impl DummyParser {
    pub fn new() -> DummyParser {
        msg_debug!("DummyParser: new()");
        DummyParser
    }
}

impl RustParser for DummyParser {
    fn process(&self, msg: &mut LogMessage, input: &str) -> bool {
        msg_debug!("DummyParser: process(input='{}')", input);
        msg.set_value("dummy_key", "value");
        false
    }

    fn init(&mut self) -> bool {
        msg_debug!("DummyParser: init()");
        true
    }

    fn set_option(&mut self, key: String, value: String) {
        msg_debug!("DummyParser: set_option(key={}, value={})", &key, &value);
    }
}

impl Drop for DummyParser {
    fn drop(&mut self) {
        msg_debug!("DummyParser: drop()");
    }
}
