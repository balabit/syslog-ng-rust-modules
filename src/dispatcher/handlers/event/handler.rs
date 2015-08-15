use std::collections::BTreeMap;

use event;
use dispatcher::request::{Request, RequestHandler};
use reactor::{self, Event};

use dispatcher::handlers::event::timer::TimerEventHandler;
use dispatcher::handlers::event::message::MessageHandler;

pub struct EventHandler{
    handlers: BTreeMap<event::EventHandler, Box<reactor::EventHandler<event::Event, Handler=event::EventHandler>>>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let timer_handler = Box::new(TimerEventHandler::new());
        let message_handler = Box::new(MessageHandler::new());
        let mut handler = EventHandler{
            handlers: BTreeMap::new()
        };
        handler.register_handler(timer_handler);
        handler.register_handler(message_handler);
        handler
    }

    fn register_handler(&mut self, handler: Box<reactor::EventHandler<event::Event, Handler=event::EventHandler>>) {
        self.handlers.insert(handler.handler(), handler);
    }
}

impl reactor::EventHandler<Request> for EventHandler {
    type Handler = RequestHandler;
    fn handle_event(&mut self, event: Request) {
        if let Request::Event(event) = event {
            println!("Event recvd");
            if let Some(handler) = self.handlers.get_mut(&event.handler()) {
                handler.handle_event(event);
            }
        } else {
            unreachable!("An EventHandler should only receive Event events");
        }
    }
    fn handler(&self) -> Self::Handler {
        RequestHandler::Event
    }
}
