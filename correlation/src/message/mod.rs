// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::BTreeMap;

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

impl Message {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_owned(), value.to_owned());
    }
}
