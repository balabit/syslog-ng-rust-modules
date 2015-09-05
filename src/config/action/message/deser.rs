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
