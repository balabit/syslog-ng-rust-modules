pub use self::message::MessageAction;

pub mod message;
mod deser;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Message(self::message::MessageAction),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecCondition {
    pub on_opened: bool,
    pub on_closed: bool,
}

impl ExecCondition {
    pub fn new() -> ExecCondition {
        Default::default()
    }
}

impl Default for ExecCondition {
    fn default() -> ExecCondition {
        ExecCondition {on_opened: false, on_closed: true}
    }
}
