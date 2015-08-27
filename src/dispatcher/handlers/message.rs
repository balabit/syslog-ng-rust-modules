use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use context;
use dispatcher::request::{InternalRequest, Request, RequestHandler};
use dispatcher::response::ResponseHandler;
use dispatcher::Response;
use context::event::EventHandler;
use message::{PatternId};
use reactor;

pub struct MessageEventHandler {
    handlers: HashMap<PatternId, Vec<Rc<RefCell<Box<context::event::EventHandler<InternalRequest>>>>>>,
    keyless_handlers: Vec<Rc<RefCell<Box<context::event::EventHandler<InternalRequest>>>>>,
    response_handler: Rc<RefCell<Box<ResponseHandler<Response>>>>,
}

impl MessageEventHandler {
    pub fn new(response_handler: Rc<RefCell<Box<ResponseHandler<Response>>>>) -> MessageEventHandler {
        MessageEventHandler{
            handlers: HashMap::new(),
            keyless_handlers: Vec::new(),
            response_handler: response_handler
        }
    }

    pub fn register_handler(&mut self, handler: Rc<RefCell<Box<context::event::EventHandler<InternalRequest>>>>) {
        let cloned_handler = handler.clone();
        if handler.borrow().handlers().is_empty() {
            self.keyless_handlers.push(cloned_handler);
        } else {
            for key in cloned_handler.borrow().handlers() {
                println!("key: {:?}", key);
                let handlers = self.handlers.entry(key.clone()).or_insert(Vec::new());
                handlers.push(cloned_handler.clone());
            }
        }
    }
}

impl reactor::EventHandler<InternalRequest> for MessageEventHandler {
    type Handler = RequestHandler;
    fn handle_event(&mut self, event: InternalRequest) {
        println!("map: {:?}", self.handlers.len());
        if let Request::Message(event) = event {
            println!("message event");
            if let Some(name) = event.name() {
                println!("message has a name");
                if let Some(handlers) = self.handlers.get_mut(name) {
                    for i in handlers.iter_mut() {
                        if let Some(result) = i.borrow_mut().handle_event(Request::Message(event.clone())) {
                            for i in result.into_iter() {
                                self.response_handler.borrow_mut().handle_response(i.into());
                            }
                        }
                    }
                }
            } else {
                println!("uuid: {:?}", event.uuid());
                if let Some(handlers) = self.handlers.get_mut(event.uuid()) {
                    for i in handlers.iter_mut() {
                        if let Some(result) = i.borrow_mut().handle_event(Request::Message(event.clone())) {
                            for i in result.into_iter() {
                                self.response_handler.borrow_mut().handle_response(i.into());
                            }
                        }
                    }
                } else {
                    println!("no handler found for this message");
                }
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
