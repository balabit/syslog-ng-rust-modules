pub mod exit {
    use dispatcher::request::{Request, RequestHandler};
    use condition::Condition;
    use reactor::EventHandler;

    pub struct ExitHandler{
        condition: Condition,
        stops: u32
    }

    impl ExitHandler {
        pub fn new(condition: Condition) -> ExitHandler {
            ExitHandler {
                condition: condition,
                stops: 0
            }
        }
    }

    impl EventHandler<Request> for ExitHandler {
        type Handler = RequestHandler;
        fn handle_event(&mut self, event: Request) {
            if let Request::Exit = event {
                self.stops += 1;

                if self.stops >= 2 {
                    self.condition.activate();
                }
            } else {
                unreachable!("An ExitHandler should only receive Exit events");
            }
        }
        fn handler(&self) -> Self::Handler {
            RequestHandler::Exit
        }
    }
}

pub mod event {
    use std::collections::BTreeMap;

    use dispatcher::request::{Request, RequestHandler};
    use reactor::{self, EventHandler, Event};

    use super::timer::TimerEventHandler;
    use self::message::MessageHandler;

    pub struct Handler{
        handlers: BTreeMap<::EventHandler, Box<reactor::EventHandler<::Event, Handler=::EventHandler>>>,
    }

    impl Handler {
        pub fn new() -> Handler {
            let timer_handler = Box::new(TimerEventHandler::new());
            let message_handler = Box::new(MessageHandler::new());
            let mut handler = Handler{
                handlers: BTreeMap::new()
            };
            handler.register_handler(timer_handler);
            handler.register_handler(message_handler);
            handler
        }

        fn register_handler(&mut self, handler: Box<reactor::EventHandler<::Event, Handler=::EventHandler>>) {
            self.handlers.insert(handler.handler(), handler);
        }
    }

    impl reactor::EventHandler<Request> for Handler {
        type Handler = RequestHandler;
        fn handle_event(&mut self, event: Request) {
            if let Request::Event(event) = event {
                println!("Event recvd");
                if let Some(handler) = self.handlers.get_mut(&event.handler()) {
                    handler.handle_event(event);
                }
            } else {
                unreachable!("An Handler should only receive Event events");
            }
        }
        fn handler(&self) -> Self::Handler {
            RequestHandler::Event
        }
    }

    pub mod message {
        use std::collections::BTreeMap;
        use std::rc::Rc;
        use std::cell::RefCell;

        use action;
        use context;
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

        impl reactor::EventHandler<::Event> for MessageHandler {
            type Handler = ::EventHandler;
            fn handle_event(&mut self, event: ::Event) {
                if let ::Event::Message(event) = event {
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
                ::EventHandler::Message
            }
        }
    }
}

mod timer {
    use std::cell::RefCell;
    use std::rc::Rc;

    use context::Context;
    use dispatcher::request::{Request, RequestHandler};
    use context::EventHandler;
    use context;
    use Event;
    use reactor;
    use TimerEvent;
    use action::ExecResult;

    pub struct TimerEventHandler {
        contexts: Vec<Rc<RefCell<Box<EventHandler<TimerEvent>>>>>
    }

    impl TimerEventHandler {
        pub fn new() -> TimerEventHandler {
            TimerEventHandler {
                contexts: Vec::new()
            }
        }
    }

    impl reactor::EventHandler<Event> for TimerEventHandler {
        type Handler = ::EventHandler;
        fn handle_event(&mut self, event: Event) {
            if let ::Event::Timer(event) = event {
                println!("timer event");
                for i in self.contexts.iter_mut() {
                    i.borrow_mut().handle_event(event);
                }
            } else {
                unreachable!("TimerEventHandler should only handle Timer events");
            }
        }
        fn handler(&self) -> Self::Handler {
            ::EventHandler::Timer
        }
    }
}
