use handlebars::Template;
use std::collections::BTreeMap;

use super::ActionType;
use super::ExecCondition;

mod deser;
mod builder;

pub use self::builder::MessageActionBuilder;

#[derive(Clone, Debug, PartialEq)]
pub struct MessageAction {
    pub uuid: String,
    pub name: Option<String>,
    pub message: Template,
    pub values: BTreeMap<String, Template>,
    pub when: ExecCondition,
    pub inject_mode: InjectMode,
}

impl MessageAction {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn message(&self) -> &Template {
        &self.message
    }
    pub fn values(&self) -> &BTreeMap<String, Template> {
        &self.values
    }
    pub fn inject_mode(&self) -> &InjectMode {
        &self.inject_mode
    }
}

impl From<MessageAction> for super::ActionType {
    fn from(action: MessageAction) -> super::ActionType {
        super::ActionType::Message(action)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InjectMode {
    Log,
    Forward,
    Loopback,
}

impl Default for InjectMode {
    fn default() -> InjectMode {
        InjectMode::Log
    }
}
