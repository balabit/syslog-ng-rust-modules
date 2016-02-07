// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use handlebars::Template;
use uuid::Uuid;

use config::action::ActionType;
use conditions::Conditions;

mod deser;
pub mod action;

pub struct ContextConfig {
    pub name: Option<String>,
    pub uuid: Uuid,
    pub conditions: Conditions,
    pub context_id: Option<Template>,
    pub actions: Vec<ActionType>,
}

pub struct ContextConfigBuilder {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    context_id: Option<Template>,
    actions: Vec<ActionType>,
}

impl ContextConfigBuilder {
    pub fn new(uuid: Uuid, conditions: Conditions) -> ContextConfigBuilder {
        ContextConfigBuilder {
            name: None,
            uuid: uuid,
            conditions: conditions,
            context_id: None,
            actions: Vec::new(),
        }
    }

    pub fn context_id(mut self, context_id: Option<Template>) -> ContextConfigBuilder {
        self.context_id = context_id;
        self
    }

    pub fn actions(mut self, actions: Vec<ActionType>) -> ContextConfigBuilder {
        self.actions = actions;
        self
    }

    pub fn name(mut self, name: String) -> ContextConfigBuilder {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> ContextConfig {
        ContextConfig {
            name: self.name,
            uuid: self.uuid,
            conditions: self.conditions,
            context_id: self.context_id,
            actions: self.actions,
        }
    }
}
