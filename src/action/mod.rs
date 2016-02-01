use config::action::ActionType;
use state::State;
use dispatcher::response::ResponseSender;
use context::base::BaseContext;

pub mod message;

pub use self::message::Alert;

pub trait Action {
    fn on_opened(&self, state: &State, context: &BaseContext, &mut ResponseSender);
    fn on_closed(&self, state: &State, context: &BaseContext, &mut ResponseSender);
}

pub fn from_config(config: ActionType) -> Box<Action> {
    match config {
        ActionType::Message(action) => Box::new(self::message::MessageAction::new(action)),
    }
}
