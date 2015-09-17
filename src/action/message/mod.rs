use action::Action;
use config;
use context::base::BaseContext;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use message::{
    Message,
    MessageBuilder
};

use handlebars::{
    Template
};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use state::State;

#[cfg(test)]
mod test;

pub const CONTEXT_UUID: &'static str = ".context.uuid";
pub const CONTEXT_NAME: &'static str = ".context.name";

pub struct MessageAction {
    sender: Rc<RefCell<Box<ResponseSender<Response>>>>,
    uuid: String,
    name: Option<String>,
    message: Template,
    values: BTreeMap<String, String>
}

impl MessageAction {
    pub fn new(sender: Rc<RefCell<Box<ResponseSender<Response>>>>, action: config::action::MessageAction) -> MessageAction {
        let config::action::MessageAction { uuid, name, message, values } = action;
        MessageAction {
            sender: sender,
            uuid: uuid,
            name: name,
            message: message,
            values: values
        }
    }
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
        let name = self.name.as_ref().map(|name| name.borrow());
        let mut message = MessageBuilder::new(&self.uuid, "moricka message")
                        .name(name)
                        .values(self.values.clone())
                        .build();
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
