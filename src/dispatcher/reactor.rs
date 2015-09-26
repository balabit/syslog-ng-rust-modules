use std::collections::BTreeMap;

use context::ContextMap;
use condition::Condition;
use dispatcher::demux::Demultiplexer;
use dispatcher::request::{RequestHandler, InternalRequest, ExternalRequest};
use reactor::{Event, EventDemultiplexer, EventHandler, Reactor};

pub struct RequestReactor {
    handlers: BTreeMap<RequestHandler, Box<EventHandler<InternalRequest, ContextMap>>>,
    demultiplexer: Demultiplexer<ExternalRequest>,
    exit_condition: Condition,
    context_map: ContextMap
}

impl RequestReactor {
    pub fn new(demultiplexer: Demultiplexer<ExternalRequest>, exit_condition: Condition, context_map: ContextMap) -> RequestReactor {
        RequestReactor {
            demultiplexer: demultiplexer,
            exit_condition: exit_condition,
            context_map: context_map,
            handlers: BTreeMap::new()
        }
    }
}

impl Reactor<ContextMap> for RequestReactor {
    type Event = InternalRequest;
    fn handle_events(&mut self) {
        while !self.exit_condition.is_active() {
            if let Some(request) = self.demultiplexer.select() {
                trace!("RequestReactor: got event");
                if let Some(handler) = self.handlers.get_mut(&request.handler()) {
                    handler.handle_event(request, &mut self.context_map);
                } else {
                    trace!("RequestReactor: no handler found for event");
                }
            } else {
                break;
            }
        }
    }
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event, ContextMap>>) {
        self.handlers.insert(handler.handler(), handler);
    }
    fn remove_handler_by_handler(&mut self, handler: &RequestHandler) {
        self.handlers.remove(handler);
    }
}
