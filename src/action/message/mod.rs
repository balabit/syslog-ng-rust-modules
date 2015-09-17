use action::Action;
use config;
use context::base::BaseContext;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use message::Message;
use std::cell::RefCell;
use std::rc::Rc;
use state::State;

#[cfg(test)]
mod test;

pub const CONTEXT_UUID: &'static str = ".context.uuid";
pub const CONTEXT_NAME: &'static str = ".context.name";

pub struct MessageAction {
    pub sender: Rc<RefCell<Box<ResponseSender<Response>>>>,
    pub action: config::action::MessageAction
}

#[derive(Debug)]
pub struct MessageResponse {
    message: Message,
}

impl MessageResponse {
    pub fn message(&self) -> &Message {
        &self.message
    }
}

impl Action for MessageAction {
    fn execute(&self, _state: &State, _context: &BaseContext) {
        trace!("MessageAction: executed");
        let mut message = Message::from(&self.action);
        message.insert(".context.uuid", &_context.uuid().to_hyphenated_string());
        message.insert(".context.len", &_state.messages().len().to_string());
        if let Some(name) = _context.name() {
            message.insert(".context.name", name);
        }
        let response = MessageResponse {
            message: message,
        };
        self.sender.borrow_mut().send_response(Response::Message(response));
    }
}
