use super::ActionType;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageAction;

impl MessageAction {
    pub fn new() -> MessageAction {
        MessageAction
    }
}

impl From<MessageAction> for super::ActionType {
    fn from(action: MessageAction) -> super::ActionType {
        super::ActionType::Message(action)
    }
}

mod deser {

use super::MessageAction;
use serde;
use serde::de::Deserialize;

impl serde::Deserialize for MessageAction {
    fn deserialize<D>(deserializer: &mut D) -> Result<MessageAction, D::Error>
        where D: serde::de::Deserializer
    {
        deserializer.visit_unit_struct("MessageAction", MessageActionVisitor)
    }
}

struct MessageActionVisitor;

impl serde::de::Visitor for MessageActionVisitor {
    type Value = MessageAction;
    fn visit_unit<E>(&mut self) -> Result<Self::Value, E>
        where E: serde::de::Error {
        Ok(MessageAction)
    }
}
}
