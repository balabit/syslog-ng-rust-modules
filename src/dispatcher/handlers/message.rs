use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use context;
use dispatcher::request::{InternalRequest, Request, RequestHandler};
use dispatcher::response::ResponseHandler;
use dispatcher::Response;
use context::event::EventHandler;
use message::{Message, PatternId};
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
                let handlers = self.handlers.entry(key.clone()).or_insert(Vec::new());
                handlers.push(cloned_handler.clone());
            }
        }
    }

    pub fn call_handlers_by_id(&mut self, id: &PatternId, event: Rc<Message>) {
        if let Some(handlers) = self.handlers.get_mut(id) {
            for i in handlers.iter_mut() {
                if let Some(result) = i.borrow_mut().handle_event(Request::Message(event.clone())) {
                    for i in result.into_iter() {
                        self.response_handler.borrow_mut().handle_response(i.into());
                    }
                }
            }
        } else {
            println!("no handler found for id: {:?}", id);
        }
    }
}

impl reactor::EventHandler<InternalRequest> for MessageEventHandler {
    type Handler = RequestHandler;
    fn handle_event(&mut self, event: InternalRequest) {
        if let Request::Message(event) = event {
            println!("message event");
            if let Some(id) = event.name() {
                self.call_handlers_by_id(id, event.clone());
            } else {
                let id = event.uuid();
                self.call_handlers_by_id(id, event.clone());
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


#[cfg(test)]
mod test {
    use uuid::Uuid;
    use std::cell::RefCell;
    use std::rc::Rc;

    use action::ExecResult;
    use context;
    use dispatcher::request::{InternalRequest, Request};
    use dispatcher::response::ResponseHandler;
    use dispatcher::Response;
    use message::{Builder, PatternId};
    use reactor::EventHandler;

    use super::MessageEventHandler;

    struct DummyResponseHandler(Rc<RefCell<i32>>);

    impl ResponseHandler<Response> for DummyResponseHandler {
        fn handle_response(&mut self, _: Response) {
            *self.0.borrow_mut() += 1;
        }
    }

    struct DummyEventHandler {
        counter: Rc<RefCell<i32>>,
        ids: Vec<PatternId>
    }

    impl context::event::EventHandler<InternalRequest> for DummyEventHandler {
        fn handle_event(&mut self, _: InternalRequest) -> Option<Vec<ExecResult>> {
            *self.counter.borrow_mut() += 1;
            None
        }
        fn handlers(&self) -> &[PatternId] {
            &self.ids
        }
    }

    #[test]
    fn test_given_message_event_handler_when_it_receives_a_message_then_it_calls_the_right_handler() {
        let uuid1 = "1b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
        let uuid2 = "2b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
        let response_handler_counter = Rc::new(RefCell::new(0));
        let response_handler: Box<ResponseHandler<Response>> = Box::new(DummyResponseHandler(response_handler_counter.clone()));
        let response_handler = Rc::new(RefCell::new(response_handler));
        let ids_1 = vec![ PatternId::Uuid(Uuid::parse_str(&uuid1).unwrap()) ];
        let ids_2 = vec![ PatternId::Uuid(Uuid::parse_str(&uuid2).unwrap()) ];
        let event_handler_counter_1 = Rc::new(RefCell::new(0));
        let event_handler_counter_2 = Rc::new(RefCell::new(0));
        let event_handler_1: Box<context::event::EventHandler<InternalRequest>> = Box::new(DummyEventHandler{counter: event_handler_counter_1.clone(), ids: ids_1});
        let event_handler_2: Box<context::event::EventHandler<InternalRequest>> = Box::new(DummyEventHandler{counter: event_handler_counter_2.clone(), ids: ids_2});
        let event_handler_1 = Rc::new(RefCell::new(event_handler_1));
        let event_handler_2 = Rc::new(RefCell::new(event_handler_2));
        let mut message_event_handler = MessageEventHandler::new(response_handler.clone());
        message_event_handler.register_handler(event_handler_1);
        message_event_handler.register_handler(event_handler_2);
        message_event_handler.handle_event(Request::Message(Rc::new(Builder::new(&uuid1).build())));
        assert_eq!(1, *event_handler_counter_1.borrow());
        message_event_handler.handle_event(Request::Message(Rc::new(Builder::new(&uuid1).build())));
        assert_eq!(2, *event_handler_counter_1.borrow());
        assert_eq!(0, *event_handler_counter_2.borrow());
        assert_eq!(0, *response_handler_counter.borrow());
    }
}
