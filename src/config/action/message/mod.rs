use std::collections::BTreeMap;

use super::ActionType;

mod deser;
mod builder;

pub use self::builder::MessageActionBuilder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageAction {
    uuid: String,
    name: Option<String>,
    values: BTreeMap<String, String>
}

impl MessageAction {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
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
