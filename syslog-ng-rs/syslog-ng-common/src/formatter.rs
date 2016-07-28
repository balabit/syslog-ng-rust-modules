// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::fmt::Write;

/// Applies transformations to key-value pairs.
///
/// # Examples
/// ```
/// # use syslog_ng_common::MessageFormatter;
///
/// let mut formatter = MessageFormatter::new();
/// formatter.set_prefix("foo.");
/// assert_eq!(formatter.format("key", "value"), ("foo.key", "value"));
/// ```
#[derive(Clone)]
pub struct MessageFormatter {
    buffer: String,
    prefix: Option<String>,
}
impl MessageFormatter {
    /// Creates a new MessageFormatter without any transformations.
    pub fn new() -> MessageFormatter {
        MessageFormatter {
            buffer: String::new(),
            prefix: None,
        }
    }
    /// Sets a `prefix` is applied to every `key` during a `format()` call.
    pub fn set_prefix<S: Into<String>>(&mut self, prefix: S) {
        self.prefix = Some(prefix.into());
    }

    /// Formats the given `key` and/or `value` parameters and returns the formatted pair as a tuple.
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
