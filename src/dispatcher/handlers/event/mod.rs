use std::collections::BTreeMap;

use dispatcher::request::{Request, RequestHandler};
use reactor::{self, EventHandler, Event};

use self::timer::TimerEventHandler;
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

pub mod message;
pub mod timer;
