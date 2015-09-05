pub use self::message::MessageAction;

pub mod message;
mod deser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActionType {
    Message(self::message::MessageAction)
}
