use dispatcher::request::{Request, RequestHandle};
use reactor::{EventHandler, SharedData};

pub struct TimerEventHandler;

impl TimerEventHandler {
    pub fn new() -> TimerEventHandler {
        TimerEventHandler
    }
}

impl<'a> EventHandler<Request, SharedData<'a>> for TimerEventHandler {
    fn handle_event(&mut self, event: Request, data: &mut SharedData) {
        for i in data.map.contexts_mut() {
            i.on_event(event.clone(), data.responder);
        }
    }
    fn handle(&self) -> RequestHandle {
        RequestHandle::Timer
    }
}
