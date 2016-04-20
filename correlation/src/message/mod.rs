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
    pub uuid: Vec<u8>,
    pub name: Option<Vec<u8>>,
    pub message: Vec<u8>,
    pub values: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Event for Message {
    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.values.get(key).map(|value| &value[..])
    }
    fn ids(&self) -> EventIds {
        EventIds {
            uuid: self.uuid().borrow(),
            name: self.name().map(|name| name.borrow())
        }
    }

    fn new(uuid: &[u8], message: &[u8]) -> Self {
        Message {
            uuid: uuid.to_vec(),
            message: message.to_vec(),
            name: None,
            values: BTreeMap::new()
        }
    }
    fn set_name(&mut self, name: Option<&[u8]>) {
        self.name = name.map(|name| name.to_vec());
    }
    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.values.insert(key.to_vec(), value.to_vec());
    }
    fn set_message(&mut self, message: &[u8]) {
        self.message = message.to_vec();
    }
    fn message(&self) -> &[u8] {
        self.message.borrow()
    }
    fn uuid(&self) -> &[u8] {
        &self.uuid
    }
    fn name(&self) -> Option<&[u8]> {
        self.name.as_ref().map(|name| name.borrow())
    }
}
