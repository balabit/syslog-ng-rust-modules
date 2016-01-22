// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::fmt::Write;

#[derive(Clone)]
pub struct MessageFormatter {
    buffer: String,
    prefix: Option<String>,
}

impl MessageFormatter {
    pub fn new() -> MessageFormatter {
        MessageFormatter {
            buffer: String::new(),
            prefix: None,
        }
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = Some(prefix)
    }

    pub fn format<'a, 'b, 'c>(&'a mut self, key: &'b str, value: &'c str) -> (&'a str, &'c str) {
        self.buffer.clear();
        self.apply_prefix(key);
        (&self.buffer, value)
    }

    fn apply_prefix(&mut self, key: &str) {
        match self.prefix.as_ref() {
            Some(prefix) => {
                let _ = self.buffer.write_str(prefix);
                let _ = self.buffer.write_str(key);
            }
            None => {
                let _ = self.buffer.write_str(key);
            }
        };
    }
}
