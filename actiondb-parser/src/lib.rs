#[macro_use]
extern crate log;

#[macro_use]
extern crate syslog_ng_sys;
extern crate actiondb;

use std::borrow::Borrow;

use actiondb::matcher::Matcher;

use syslog_ng_sys::{RustParser,
                    LogMessage};

#[derive(Clone)]
pub struct ActiondbParser {
    matcher: Option<Matcher>
}

impl ActiondbParser {
    pub fn new() -> ActiondbParser {
        debug!("ActiondbParser: new()");
        ActiondbParser{ matcher: None }
    }
}

impl RustParser for ActiondbParser {
    fn process(&self, msg: &mut LogMessage, input: &str) -> bool {
        debug!("ActiondbParser: process(input='{}')", input);
        let parse_result = self.matcher.as_ref().unwrap().parse(input);

        if let Some(kv_pairs) = parse_result {
            debug!("parser matched");
            for (key, value) in kv_pairs {
                msg.set_value(key, value);
            }
            true
        } else {
            false
        }
    }

    fn init(&mut self) -> bool {
        debug!("ActiondbParser: init()");
        if self.matcher.is_none() {
            error!("ActiondbParser: not all required parameters are set");
            false
        } else {
            true
        }
    }

    fn set_option(&mut self, key: String, value: String) {
        debug!("ActiondbParser: set_option(key={}, value={})", &key, &value);

        match key.borrow() {
            "pattern_file" => {
                let matcher = Matcher::from_file(&value);

                if matcher.is_ok() {
                    self.matcher = matcher.ok();
                } else {
                    error!("ActiondbParser: failed to set 'pattern_file'");
                }
            },
            _ => {
                debug!("ActiondbParser not supported key: {:?}", key) ;
            }
        };
    }

    fn boxed_clone(&self) -> Box<RustParser> {
        Box::new(self.clone())
    }
}
