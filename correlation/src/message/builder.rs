// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::BTreeMap;
use std::convert::Into;
use super::Message;

pub struct MessageBuilder {
    uuid: Vec<u8>,
    name: Option<Vec<u8>>,
    message: Vec<u8>,
    values: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl MessageBuilder {
    pub fn new<S: Into<Vec<u8>>>(uuid: S, message: S) -> MessageBuilder {
        MessageBuilder {
            uuid: uuid.into(),
            name: None,
            message: message.into(),
            values: BTreeMap::new(),
        }
    }

    pub fn name<S: Into<Vec<u8>>>(&mut self, name: Option<S>) -> &mut MessageBuilder {
        self.name = name.map(|name| name.into());
        self
    }

    pub fn values(&mut self, values: BTreeMap<Vec<u8>, Vec<u8>>) -> &mut MessageBuilder {
        self.values = values;
        self
    }

    pub fn pair(&mut self, key: &[u8], value: &[u8]) -> &mut MessageBuilder {
        self.values.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn build(&self) -> Message {
        Message {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            message: self.message.clone(),
            values: self.values.clone(),
        }
    }
}
