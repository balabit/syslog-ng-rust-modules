use std::cell::RefCell;
use std::rc::Rc;

use context::Context;
use dispatcher::request::{InternalRequest, Request, RequestHandler};
use context::event::EventHandler;
use context;
use reactor;
use TimerEvent;
use action::ExecResult;

pub struct TimerEventHandler {
    contexts: Vec<Rc<RefCell<Box<EventHandler<InternalRequest>>>>>
}

impl TimerEventHandler {
    pub fn new() -> TimerEventHandler {
        TimerEventHandler {
            contexts: Vec::new()
        }
    }
}

impl reactor::EventHandler<InternalRequest> for TimerEventHandler {
    type Handler = RequestHandler;
    fn handle_event(&mut self, event: InternalRequest) {
            println!("timer event");
            let event: InternalRequest = event.into();
            for i in self.contexts.iter_mut() {
                //i.borrow_mut().handle_event(event.clone());
            }
    }
    fn handler(&self) -> Self::Handler {
        RequestHandler::Timer
    }
}
