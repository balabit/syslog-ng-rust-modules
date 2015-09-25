use std::cell::RefCell;
use std::rc::Rc;

use dispatcher::request::{InternalRequest, RequestHandler};
use context::event::EventHandler;
use reactor;

pub struct TimerEventHandler {
    contexts: Vec<Rc<RefCell<Box<EventHandler<InternalRequest>>>>>,
}

impl TimerEventHandler {
    pub fn new() -> TimerEventHandler {
        TimerEventHandler {
            contexts: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Rc<RefCell<Box<EventHandler<InternalRequest>>>>) {
        self.contexts.push(handler);
    }
}

impl reactor::EventHandler<InternalRequest> for TimerEventHandler {
    fn handle_event(&mut self, event: InternalRequest) {
        for i in self.contexts.iter_mut() {
            i.borrow_mut().handle_event(event.clone());
        }
    }
    fn handler(&self) -> RequestHandler {
        RequestHandler::Timer
    }
}
