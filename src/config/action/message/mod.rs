use std::collections::BTreeMap;

use super::ActionType;

mod deser;
mod builder;

pub use self::builder::Builder;

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
}

impl From<MessageAction> for super::ActionType {
    fn from(action: MessageAction) -> super::ActionType {
        super::ActionType::Message(action)
    }
}
