use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;

use action;
use context;
use event;
use Message;
use dispatcher::request::{Request, RequestHandler};
use context::EventHandler;
use reactor::{self, Event};

pub struct MessageHandler {
    handlers: BTreeMap<String, Vec<Rc<RefCell<Box<context::EventHandler<Rc<Message> >>>>>>,
    keyless_handlers: Vec<Rc<RefCell<Box<context::EventHandler<Rc<Message> >>>>>,
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        MessageHandler{
            handlers: BTreeMap::new(),
            keyless_handlers: Vec::new()
        }
    }

    fn register_handler(&mut self, handler: Box<context::EventHandler<Rc<Message>>>) {
        if handler.handlers().is_empty() {
            let handler = Rc::new(RefCell::new(handler));
            self.keyless_handlers.push(handler);
        } else {
            let refcounted_handler = Rc::new(RefCell::new(handler));
            let cloned_handler = refcounted_handler.clone();
            for key in cloned_handler.borrow().handlers() {
                let handlers = self.handlers.entry(key.clone()).or_insert(Vec::new());
                handlers.push(cloned_handler.clone());
            }
        }
    }
}

impl reactor::EventHandler<event::Event> for MessageHandler {
    type Handler = event::EventHandler;
    fn handle_event(&mut self, event: event::Event) {
        if let event::Event::Message(event) = event {
            let event = Rc::new(event);

            println!("message event");
            if let Some(handlers) = self.handlers.get_mut(event.uuid()) {
                for i in handlers.iter_mut() {
                    i.borrow_mut().handle_event(event.clone());
                }
            } else {
                println!("no handler found for this message");
            }

            for i in self.keyless_handlers.iter_mut() {
                i.borrow_mut().handle_event(event.clone());
            }
        } else {
            unreachable!("MessageEventHandler should only handle Message events");
        }
    }
    fn handler(&self) -> Self::Handler {
        event::EventHandler::Message
    }
}
