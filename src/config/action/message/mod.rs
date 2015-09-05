use super::ActionType;

mod deser;

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
