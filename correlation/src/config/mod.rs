// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use uuid::Uuid;
use std::collections::BTreeMap;

use config::action::ActionType;
use config::action::message::MessageAction;
use conditions::Conditions;
use Event;
use TemplateFactory;
use CompileError;

mod deser;
pub mod action;

pub struct ContextConfig<T> {
    pub name: Option<String>,
    pub uuid: Uuid,
    pub conditions: Conditions,
    pub context_id: Option<Vec<String>>,
    pub actions: Vec<ActionType<T>>,
    pub patterns: Vec<String>
}

pub fn compile_templates<T, E, TF>(original: Vec<ContextConfig<T>>, factory: &TF) -> Result<Vec<ContextConfig<TF::Template>>, CompileError>
    where T: AsRef<[u8]>, E: Event, TF: TemplateFactory<E> {
    let mut new_contexts: Vec<ContextConfig<TF::Template>> = Vec::new();
    for context in original {
        let ContextConfig {name, uuid, conditions, context_id, actions, patterns} = context;
        let mut new_actions: Vec<ActionType<TF::Template>> = Vec::new();

        for action in actions {
            let ActionType::Message(message_action) = action;
            let MessageAction {uuid, name, message, values, when, inject_mode} = message_action;
            let new_message = try!(factory.compile(message.as_ref()));
            let mut new_values = BTreeMap::new();

            for (key, value) in values {
                let value = try!(factory.compile(value.as_ref()));
                new_values.insert(key, value);
            }

            let action: MessageAction<TF::Template> = MessageAction {
                uuid: uuid,
                name: name,
                message: new_message,
                values: new_values,
                when: when,
                inject_mode: inject_mode
            };
            new_actions.push(ActionType::Message(action));
        }

        let config = ContextConfig {
            name: name,
            uuid: uuid,
            conditions: conditions,
            context_id: context_id,
            actions: new_actions,
            patterns: patterns
        };

        new_contexts.push(config);
    }
    Ok(new_contexts)
}

pub struct ContextConfigBuilder<T> {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    context_id: Option<Vec<String>>,
    actions: Vec<ActionType<T>>,
    patterns: Vec<String>
}

impl<T> ContextConfigBuilder<T> {
    pub fn new(uuid: Uuid, conditions: Conditions) -> ContextConfigBuilder<T> {
        ContextConfigBuilder {
            name: None,
            uuid: uuid,
            conditions: conditions,
            context_id: None,
            actions: Vec::new(),
            patterns: Vec::new()
        }
    }

    pub fn context_id(mut self, context_id: Option<Vec<String>>) -> ContextConfigBuilder<T> {
        self.context_id = context_id;
        self
    }

    pub fn actions(mut self, actions: Vec<ActionType<T>>) -> ContextConfigBuilder<T> {
        self.actions = actions;
        self
    }

    pub fn name(mut self, name: String) -> ContextConfigBuilder<T> {
        self.name = Some(name);
        self
    }

    pub fn patterns(mut self, patterns: Vec<String>) -> ContextConfigBuilder<T> {
        self.patterns = patterns;
        self
    }

    pub fn build(self) -> ContextConfig<T> {
        ContextConfig {
            name: self.name,
            uuid: self.uuid,
            conditions: self.conditions,
            context_id: self.context_id,
            actions: self.actions,
            patterns: self.patterns
        }
    }
}
