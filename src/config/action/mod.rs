pub use self::message::MessageAction;

pub mod message;
mod deser;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Message(self::message::MessageAction),
}

pub const ON_CLOSED_DEFAULT: Option<bool> = Some(true);
pub const ON_OPENED_DEFAULT: Option<bool> = None;

#[derive(Clone, Debug, PartialEq)]
pub struct ExecCondition {
    pub on_opened: Option<bool>,
    pub on_closed: Option<bool>,
}
