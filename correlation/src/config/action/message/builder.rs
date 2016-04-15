// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use super::MessageAction;
use super::InjectMode;
use config::action::ExecCondition;

use std::collections::BTreeMap;

pub struct MessageActionBuilder<T> {
    uuid: String,
    name: Option<String>,
    message: T,
    values: BTreeMap<String, T>,
    when: ExecCondition,
    inject_mode: InjectMode,
}

impl<T> MessageActionBuilder<T> {
    pub fn new<U: Into<String>, M: Into<T>>(uuid: U, message: M) -> MessageActionBuilder<T> {
        MessageActionBuilder {
            uuid: uuid.into(),
            name: None,
            message: message.into(),
            values: BTreeMap::default(),
            when: ExecCondition::default(),
            inject_mode: InjectMode::default(),
        }
    }

    pub fn name<S: Into<String>>(mut self, name: Option<S>) -> MessageActionBuilder<T> {
        self.name = name.map(|name| name.into());
        self
    }

    pub fn when(mut self, when: ExecCondition) -> MessageActionBuilder<T> {
        self.when = when;
        self
    }

    pub fn values(mut self, values: BTreeMap<String, T>) -> MessageActionBuilder<T> {
        self.values = values;
        self
    }

    pub fn pair<K: Into<String>, V: Into<T>>(mut self, key: K, value: V) -> MessageActionBuilder<T> {
        self.values.insert(key.into(), value.into());
        self
    }

    pub fn inject_mode(mut self, mode: InjectMode) -> MessageActionBuilder<T> {
        self.inject_mode = mode;
        self
    }

    pub fn build(self) -> MessageAction<T> {
        MessageAction {
            uuid: self.uuid,
            name: self.name,
            message: self.message,
            values: self.values,
            when: self.when,
            inject_mode: self.inject_mode,
        }
    }
}
