use std::collections::BTreeMap;

use context::ContextMap;
use dispatcher::demux::Demultiplexer;
use dispatcher::request::{RequestHandler, InternalRequest, ExternalRequest};
use reactor::{Event, EventDemultiplexer, EventHandler, Reactor};

pub struct RequestReactor {
    handlers: BTreeMap<RequestHandler, Box<EventHandler<InternalRequest, ContextMap>>>,
    demultiplexer: Demultiplexer<ExternalRequest>,
    context_map: ContextMap,
}

impl RequestReactor {
    pub fn new(demultiplexer: Demultiplexer<ExternalRequest>,
               context_map: ContextMap)
               -> RequestReactor {
        RequestReactor {
            demultiplexer: demultiplexer,
            context_map: context_map,
            handlers: BTreeMap::new(),
        }
    }
}

impl Reactor<ContextMap> for RequestReactor {
    type Event = InternalRequest;
    fn handle_events(&mut self) {
        while let Some(request) = self.demultiplexer.select() {
            trace!("RequestReactor: got event");
            if let Some(handler) = self.handlers.get_mut(&request.handler()) {
                handler.handle_event(request, &mut self.context_map);
            } else {
                trace!("RequestReactor: no handler found for event");
            }
        }
    }
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event, ContextMap>>) {
        self.handlers.insert(handler.handle(), handler);
    }
    fn remove_handler_by_handle(&mut self, handler: &RequestHandler) {
        self.handlers.remove(handler);
    }
}
