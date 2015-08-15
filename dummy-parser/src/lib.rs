#[macro_use]
extern crate log;

#[macro_use]
extern crate syslog_ng_sys;

use syslog_ng_sys::{RustParser,
                    LogMessage};

#[derive(Clone)]
pub struct DummyParser;

impl DummyParser {
    pub fn new() -> DummyParser {
        debug!("DummyParser: new()");
        DummyParser
    }
}

impl RustParser for DummyParser {
    fn process(&mut self, msg: &mut LogMessage, input: &str) -> bool {
        debug!("DummyParser: process(input='{}')", input);
        msg.set_value("dummy_key", "value");
        false
    }

    fn init(&mut self) -> bool {
        debug!("DummyParser: init()");
        true
    }

    fn set_option(&mut self, key: String, value: String) {
        debug!("DummyParser: set_option(key={}, value={})", &key, &value);
    }

    fn boxed_clone(&self) -> Box<RustParser> {
        Box::new(self.clone())
    }
}

impl Drop for DummyParser {
    fn drop(&mut self) {
        debug!("DummyParser: drop()");
    }
}
