pub use self::message::MessageAction;

pub mod message;
mod deser;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Message(self::message::MessageAction),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecCondition {
    pub on_opened: Option<bool>,
    pub on_closed: Option<bool>,
}
