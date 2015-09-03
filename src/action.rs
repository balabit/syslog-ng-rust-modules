use config::action::ActionType;
use state::State;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use context::base::BaseContext;

use std::cell::RefCell;
use std::rc::Rc;

pub use self::message::MessageResponse;

pub trait Action {
    fn execute(&self, state: &State, context: &BaseContext);
}

pub fn from_config(config: ActionType, _sender: Rc<RefCell<Box<ResponseSender<Response>>>>) -> Box<Action> {
    match config {
        ActionType::Message(action) => Box::new(self::message::MessageActionType {sender: _sender, action: action})
    }
}

mod message {
    use action::Action;
    use config;
    use context::base::BaseContext;
    use dispatcher::Response;
    use dispatcher::response::ResponseSender;
    use message::{Builder, Message};
    use std::cell::RefCell;
    use std::rc::Rc;
    use state::State;

    #[derive(Clone)]
    pub struct MessageActionType {
        pub sender: Rc<RefCell<Box<ResponseSender<Response>>>>,
        pub action: config::action::MessageActionType
    }

    #[derive(Debug)]
    pub struct MessageResponse {
        message: Message,
        internal: bool
    }

    impl Action for MessageActionType {
        fn execute(&self, _state: &State, _context: &BaseContext) {
            let mut message = Builder::new("d6621bd6-4898-4b8c-a4ff-36d0eed7d8dc")
                                    .pair(".context.uuid".to_string(), _context.uuid().to_hyphenated_string())
                                    .pair(".context.len".to_string(), _state.messages().len().to_string())
                                    .build();
            if let Some(name) = _context.name() {
                message.insert(".context.name", name);
            }
            let response = MessageResponse {
                message: message,
                internal: true
            };
            self.sender.borrow_mut().send_response(Response::Message(response));
        }
    }
}
