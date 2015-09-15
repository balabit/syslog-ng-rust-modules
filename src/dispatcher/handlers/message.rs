use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use context;
use dispatcher::request::{InternalRequest, Request, RequestHandler};
use context::event::EventHandler;
use message::{Message};
use reactor;

pub struct MessageEventHandler {
    handlers: HashMap<String, Vec<Rc<RefCell<Box<context::event::EventHandler<InternalRequest>>>>>>,
    keyless_handlers: Vec<Rc<RefCell<Box<context::event::EventHandler<InternalRequest>>>>>,
}

impl MessageEventHandler {
    pub fn new() -> MessageEventHandler {
        MessageEventHandler{
            handlers: HashMap::new(),
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

    fn call_handlers_by_id(&mut self, id: &String, event: Rc<Message>) {
        if let Some(handlers) = self.handlers.get_mut(id) {
            for i in handlers.iter_mut() {
                i.borrow_mut().handle_event(Request::Message(event.clone()));
            }
        }
    }

    fn call_handlers_by_event(&mut self, event: Rc<Message>) {
        if let Some(id) = event.name() {
            self.call_handlers_by_id(id, event.clone());
        } else {
            let id = event.uuid();
            self.call_handlers_by_id(id, event.clone());
        }
    }

    fn call_keyless_handlers(&mut self, event: Rc<Message>) {
        for i in self.keyless_handlers.iter_mut() {
            i.borrow_mut().handle_event(Request::Message(event.clone()));
        }
    }
}

impl reactor::EventHandler<InternalRequest> for MessageEventHandler {
    fn handle_event(&mut self, event: InternalRequest) {
        if let Request::Message(event) = event {
            trace!("MessageEventHandler: handle_event()");
            self.call_handlers_by_event(event.clone());
            self.call_keyless_handlers(event.clone());
        } else {
            unreachable!("MessageEventHandler should only handle Message events");
        }
    }
    fn handler(&self) -> RequestHandler {
        RequestHandler::Message
    }
}


#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use context;
    use dispatcher::request::{InternalRequest, Request};
    use message::{MessageBuilder};
    use reactor::EventHandler;

    use super::MessageEventHandler;

    struct DummyEventHandler {
        counter: Rc<RefCell<i32>>,
        ids: Vec<String>
    }

    impl context::event::EventHandler<InternalRequest> for DummyEventHandler {
        fn handle_event(&mut self, _: InternalRequest) {
            *self.counter.borrow_mut() += 1;
        }
        fn handlers(&self) -> &[String] {
            &self.ids
        }
    }

    #[test]
    fn test_given_message_event_handler_when_it_receives_a_message_then_it_calls_the_right_handler() {
        let uuid1 = "1b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
        let uuid2 = "2b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
        let response_handler_counter = Rc::new(RefCell::new(0));
        let ids_1 = vec![ uuid1.clone() ];
        let ids_2 = vec![ uuid2.clone() ];
        let event_handler_counter_1 = Rc::new(RefCell::new(0));
        let event_handler_counter_2 = Rc::new(RefCell::new(0));
        let event_handler_1: Box<context::event::EventHandler<InternalRequest>> = Box::new(DummyEventHandler{counter: event_handler_counter_1.clone(), ids: ids_1});
        let event_handler_2: Box<context::event::EventHandler<InternalRequest>> = Box::new(DummyEventHandler{counter: event_handler_counter_2.clone(), ids: ids_2});
        let event_handler_1 = Rc::new(RefCell::new(event_handler_1));
        let event_handler_2 = Rc::new(RefCell::new(event_handler_2));
        let mut message_event_handler = MessageEventHandler::new();
        message_event_handler.register_handler(event_handler_1);
        message_event_handler.register_handler(event_handler_2);
        message_event_handler.handle_event(Request::Message(Rc::new(MessageBuilder::new(&uuid1, "message").build())));
        assert_eq!(1, *event_handler_counter_1.borrow());
        message_event_handler.handle_event(Request::Message(Rc::new(MessageBuilder::new(&uuid1, "message").build())));
        assert_eq!(2, *event_handler_counter_1.borrow());
        assert_eq!(0, *event_handler_counter_2.borrow());
        assert_eq!(0, *response_handler_counter.borrow());
    }

    #[test]
    fn test_given_message_event_handler_when_a_message_has_a_name_then_the_event_handlers_are_looked_up_by_the_name() {
        let uuid1 = "1b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
        let name = "NAME".to_string();
        let response_handler_counter = Rc::new(RefCell::new(0));
        // this is the key point: the event handler will be registered by the name
        let ids_1 = vec![ name.clone() ];
        let event_handler_counter_1 = Rc::new(RefCell::new(0));
        let event_handler_1: Box<context::event::EventHandler<InternalRequest>> = Box::new(DummyEventHandler{counter: event_handler_counter_1.clone(), ids: ids_1});
        let event_handler_1 = Rc::new(RefCell::new(event_handler_1));
        let mut message_event_handler = MessageEventHandler::new();
        message_event_handler.register_handler(event_handler_1);
        message_event_handler.handle_event(Request::Message(Rc::new(MessageBuilder::new(&uuid1, "message").name(Some(&name)).build())));
        assert_eq!(1, *event_handler_counter_1.borrow());
        message_event_handler.handle_event(Request::Message(Rc::new(MessageBuilder::new(&uuid1, "message").name(Some(&name)).build())));
        assert_eq!(2, *event_handler_counter_1.borrow());
        assert_eq!(0, *response_handler_counter.borrow());
    }
}
