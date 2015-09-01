use config::action::ActionType;
use state::State;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use context::base::BaseContext;

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Action {
    fn execute(&self, state: &State, context: &BaseContext);
    fn set_response_sender(&self, _sender: Rc<RefCell<Box<ResponseSender<Response>>>>) {}
}

pub fn from_config(config: ActionType, _sender: Rc<RefCell<Box<ResponseSender<Response>>>>) -> Box<Action> {
    match config {
        ActionType::Message(action) => Box::new(self::message::MessageActionType {sender: _sender})
    }
}

mod message {
    use action::Action;
    use config;
    use context::base::BaseContext;
    use dispatcher::Response;
    use dispatcher::response::ResponseSender;
    use std::cell::RefCell;
    use std::rc::Rc;
    use state::State;

    #[derive(Clone)]
    pub struct MessageActionType {
        pub sender: Rc<RefCell<Box<ResponseSender<Response>>>>
    }

    impl Action for MessageActionType {
        fn execute(&self, _state: &State, _context: &BaseContext) {
        }
    }
}
