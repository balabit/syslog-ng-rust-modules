// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use handlebars::Template;
use handlebars::Handlebars;
use super::MessageAction;
use super::InjectMode;
use config::action::ExecCondition;

pub struct MessageActionBuilder {
    uuid: String,
    name: Option<String>,
    message: String,
    values: Handlebars,
    when: ExecCondition,
    inject_mode: InjectMode,
}

impl MessageActionBuilder {
    pub fn new<S: Into<String>>(uuid: S, message: S) -> MessageActionBuilder {
        let values = Handlebars::new();
        MessageActionBuilder {
            uuid: uuid.into(),
            name: None,
            message: message.into(),
            values: values,
            when: ExecCondition::new(),
            inject_mode: Default::default(),
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

    pub fn values(mut self, values: Handlebars) -> MessageActionBuilder {
        self.values = values;
        self
    }

    pub fn pair<S: AsRef<str>>(mut self, key: S, value: Template) -> MessageActionBuilder {
        self.values.register_template(key.as_ref(), value);
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
