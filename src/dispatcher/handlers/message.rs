use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;

use action;
use context;
use Message;
use dispatcher::request::{InternalRequest, Request, RequestHandler};
use context::event::EventHandler;
use reactor::{self, Event};

pub struct MessageEventHandler {
    handlers: BTreeMap<String, Vec<Rc<RefCell<Box<context::event::EventHandler<InternalRequest>>>>>>,
    keyless_handlers: Vec<Rc<RefCell<Box<context::event::EventHandler<InternalRequest>>>>>,
}

impl MessageEventHandler {
    pub fn new() -> MessageEventHandler {
        MessageEventHandler{
            handlers: BTreeMap::new(),
            keyless_handlers: Vec::new()
        }
    }

    pub fn register_handler(&mut self, handler: Rc<RefCell<Box<context::event::EventHandler<InternalRequest>>>>) {
        let cloned_handler = handler.clone();
        if handler.borrow().handlers().is_empty() {
            self.keyless_handlers.push(cloned_handler);
        } else {
            for key in cloned_handler.borrow().handlers() {
                let handlers = self.handlers.entry(key.clone()).or_insert(Vec::new());
                handlers.push(cloned_handler.clone());
            }
        }
    }
}

impl reactor::EventHandler<InternalRequest> for MessageEventHandler {
    type Handler = RequestHandler;
    fn handle_event(&mut self, event: InternalRequest) {
        if let Request::Message(event) = event {
            println!("message event");
            if let Some(handlers) = self.handlers.get_mut(event.uuid()) {
                for i in handlers.iter_mut() {
                    i.borrow_mut().handle_event(Request::Message(event.clone()));
                }
            } else {
                println!("no handler found for this message");
            }

            for i in self.keyless_handlers.iter_mut() {
                i.borrow_mut().handle_event(Request::Message(event.clone()));
            }
        } else {
            unreachable!("MessageEventHandler should only handle Message events");
        }
    }
    fn handler(&self) -> Self::Handler {
        RequestHandler::Message
    }
}
