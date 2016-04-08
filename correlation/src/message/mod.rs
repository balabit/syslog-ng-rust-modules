// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::BTreeMap;
use std::borrow::Borrow;

use Event;
use EventIds;
pub use self::builder::MessageBuilder;

mod builder;
#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    uuid: String,
    name: Option<String>,
    message: String,
    values: BTreeMap<String, String>,
}

impl Event for Message {
    fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|x| x.borrow())
    }
    fn ids(&self) -> EventIds {
        EventIds {
            uuid: self.uuid().borrow(),
            name: self.name().map(|name| name.borrow())
        }
    }

    fn new(uuid: &str, message: &str) -> Self {
        Message {
            uuid: uuid.to_string(),
            message: message.to_string(),
            name: None,
            values: BTreeMap::new()
        }
    }
    fn set_name(&mut self, name: Option<&str>) {
        self.name = name.map(|name| name.to_string());
    }
    fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }
    fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }
    fn message(&self) -> &str {
        self.message.borrow()
    }
    fn uuid(&self) -> &str {
        &self.uuid
    }
    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|name| name.borrow())
    }
}
