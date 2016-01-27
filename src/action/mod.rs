use config::action::ActionType;
use state::State;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use context::base::BaseContext;

pub mod message;

pub use self::message::MessageResponse;

pub trait Action {
    fn on_opened(&self, state: &State, context: &BaseContext);
    fn on_closed(&self, state: &State, context: &BaseContext);
}

pub fn from_config(config: ActionType,
                   _sender: Box<ResponseSender<Response>>)
                   -> Box<Action> {
    match config {
        ActionType::Message(action) => Box::new(self::message::MessageAction::new(_sender, action)),
    }
}
