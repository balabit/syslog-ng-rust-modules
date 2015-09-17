use config::action::ActionType;
use state::State;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use context::base::BaseContext;

use std::cell::RefCell;
use std::rc::Rc;

pub mod message;

pub use self::message::MessageResponse;

pub trait Action {
    fn execute(&self, state: &State, context: &BaseContext);
}

pub fn from_config(config: ActionType, _sender: Rc<RefCell<Box<ResponseSender<Response>>>>) -> Box<Action> {
    match config {
        ActionType::Message(action) => Box::new(self::message::MessageAction::new(_sender, action))
    }
}
