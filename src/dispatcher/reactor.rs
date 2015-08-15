use std::collections::BTreeMap;

use condition::Condition;
use dispatcher::demux::Demultiplexer;
use dispatcher::handlers;
use dispatcher::request::{Request, RequestHandler};
use reactor::{Event, EventDemultiplexer, EventHandler, Reactor};

pub struct RequestReactor {
    handlers: BTreeMap<RequestHandler, Box<EventHandler<Request, Handler=RequestHandler>>>,
    demultiplexer: Demultiplexer<Request>,
    exit_condition: Condition
}

impl RequestReactor {
    pub fn new(demultiplexer: Demultiplexer<Request>, exit_condition: Condition) -> RequestReactor {
        RequestReactor {
            demultiplexer: demultiplexer,
            exit_condition: exit_condition,
            handlers: BTreeMap::new()
        }
    }
}

impl Reactor for RequestReactor {
    type Event = Request;
    type Handler = RequestHandler;
    fn handle_events(&mut self) {
        while !self.exit_condition.is_active() {
            if let Some(request) = self.demultiplexer.select() {
                let mut handler = self.handlers.get_mut(&request.handler()).unwrap();
                handler.handle_event(request);
            } else {
                break;
            }
        }
    }
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event, Handler=RequestHandler>>) {
        self.handlers.insert(handler.handler(), handler);
    }
    fn remove_handler(&mut self, handler: &EventHandler<Self::Event, Handler=RequestHandler>){}
}
