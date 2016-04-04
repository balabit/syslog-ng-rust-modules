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

pub struct MessageActionBuilder {
    uuid: String,
    name: Option<String>,
    message: String,
    values: BTreeMap<String, String>,
    when: ExecCondition,
    inject_mode: InjectMode,
}

impl MessageActionBuilder {
    pub fn new<S: Into<String>>(uuid: S, message: S) -> MessageActionBuilder {
        MessageActionBuilder {
            uuid: uuid.into(),
            name: None,
            message: message.into(),
            values: BTreeMap::default(),
            when: ExecCondition::default(),
            inject_mode: InjectMode::default(),
        }
    }

    pub fn name<S: Into<String>>(mut self, name: Option<S>) -> MessageActionBuilder {
        self.name = name.map(|name| name.into());
        self
    }

    pub fn when(mut self, when: ExecCondition) -> MessageActionBuilder {
        self.when = when;
        self
    }

    pub fn values(mut self, values: BTreeMap<String, String>) -> MessageActionBuilder {
        self.values = values;
        self
    }

    pub fn pair<S: Into<String>>(mut self, key: S, value: S) -> MessageActionBuilder {
        self.values.insert(key.into(), value.into());
        self
    }

    pub fn inject_mode(mut self, mode: InjectMode) -> MessageActionBuilder {
        self.inject_mode = mode;
        self
    }

    pub fn build(self) -> MessageAction {
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
