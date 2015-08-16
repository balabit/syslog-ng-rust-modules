use std::collections::BTreeMap;

use condition::Condition;
use dispatcher::demux::Demultiplexer;
use dispatcher::request::{RequestHandler, InternalRequest, ExternalRequest};
use reactor::{Event, EventDemultiplexer, EventHandler, Reactor};

pub struct RequestReactor {
    handlers: BTreeMap<RequestHandler, Box<EventHandler<InternalRequest, Handler=RequestHandler>>>,
    demultiplexer: Demultiplexer<ExternalRequest>,
    exit_condition: Condition
}

impl RequestReactor {
    pub fn new(demultiplexer: Demultiplexer<ExternalRequest>, exit_condition: Condition) -> RequestReactor {
        RequestReactor {
            demultiplexer: demultiplexer,
            exit_condition: exit_condition,
            handlers: BTreeMap::new()
        }
    }
}

impl Reactor for RequestReactor {
    type Event = InternalRequest;
    type Handler = RequestHandler;
    fn handle_events(&mut self) {
        while !self.exit_condition.is_active() {
            if let Some(request) = self.demultiplexer.select() {
                if let Some(handler) = self.handlers.get_mut(&request.handler()) {
                    handler.handle_event(request);
                } else {
                    println!("Handler not found for event");
                }
            } else {
                break;
            }
        }
    }
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event, Handler=RequestHandler>>) {
        self.handlers.insert(handler.handler(), handler);
    }
    fn remove_handler_by_handler(&mut self, handler: &RequestHandler) {
        self.handlers.remove(handler);
    }
}
