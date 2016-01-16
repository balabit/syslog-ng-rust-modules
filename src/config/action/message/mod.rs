use handlebars::Template;
use std::collections::BTreeMap;

use super::ActionType;

mod deser;
mod builder;

pub use self::builder::MessageActionBuilder;

#[derive(Clone, Debug, PartialEq)]
pub struct MessageAction {
    pub uuid: String,
    pub name: Option<String>,
    pub message: Template,
    pub values: BTreeMap<String, Template>,
    pub on_opened: Option<bool>,
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
}

impl From<MessageAction> for super::ActionType {
    fn from(action: MessageAction) -> super::ActionType {
        super::ActionType::Message(action)
    }
}
