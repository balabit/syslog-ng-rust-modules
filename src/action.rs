use state::State;
use dispatcher::response::ResponseSender;
use context::base::BaseContext;

pub use config::action::message::Alert;

pub trait Action {
    fn on_opened(&self, state: &State, context: &BaseContext, &mut ResponseSender);
    fn on_closed(&self, state: &State, context: &BaseContext, &mut ResponseSender);
}
