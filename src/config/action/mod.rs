use action::Action;
use state::State;
use context::BaseContext;
use dispatcher::response::ResponseSender;

pub use self::message::MessageAction;

pub mod message;
mod deser;

pub enum ActionType {
    Message(self::message::MessageAction),
}

impl Action for ActionType {
    fn on_opened(&self, state: &State, context: &BaseContext, responder: &mut ResponseSender) {
        match *self {
            ActionType::Message(ref action) => action.on_opened(state, context, responder),
        }
    }
    fn on_closed(&self, state: &State, context: &BaseContext, responder: &mut ResponseSender) {
        match *self {
            ActionType::Message(ref action) => action.on_closed(state, context, responder),
        }
    }
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
        ExecCondition {
            on_opened: false,
            on_closed: true,
        }
    }
}
