use dispatcher::request::{InternalRequest, RequestHandle};
use context::ContextMap;
use reactor::EventHandler;

pub struct TimerEventHandler;

impl TimerEventHandler {
    pub fn new() -> TimerEventHandler {
        TimerEventHandler
    }
}

impl EventHandler<InternalRequest, ContextMap> for TimerEventHandler {
    fn handle_event(&mut self, event: InternalRequest, data: &mut ContextMap) {
        for i in data.contexts_mut() {
            i.on_event(event.clone());
        }
    }
    fn handle(&self) -> RequestHandle {
        RequestHandle::Timer
    }
}
