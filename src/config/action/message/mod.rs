use std::borrow::Borrow;
use std::collections::BTreeMap;

use message::{
    Message,
    MessageBuilder
};
use super::ActionType;

mod deser;
mod builder;

pub use self::builder::MessageActionBuilder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageAction {
    uuid: String,
    name: Option<String>,
    message: String,
    values: BTreeMap<String, String>
}

impl MessageAction {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn message(&self) -> &String {
        &self.message
    }
    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }
}

impl From<MessageAction> for super::ActionType {
    fn from(action: MessageAction) -> super::ActionType {
        super::ActionType::Message(action)
    }
}

impl<'a> From<&'a MessageAction> for Message {
    fn from(action: &'a MessageAction) -> Message {
        let name = action.name().map(|name| name.borrow());
        MessageBuilder::new(action.uuid())
                        .name(name)
                        .values(action.values().clone())
                        .build()
    }
}
