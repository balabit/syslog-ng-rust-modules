use std::cell::RefCell;
use std::rc::Rc;

use context::Context;
use dispatcher::request::{Request, RequestHandler};
use context::event::EventHandler;
use context;
use event;
use context::event::Event;
use reactor;
use TimerEvent;
use action::ExecResult;

pub struct TimerEventHandler {
    contexts: Vec<Rc<RefCell<Box<EventHandler<context::event::Event>>>>>
}

impl TimerEventHandler {
    pub fn new() -> TimerEventHandler {
        TimerEventHandler {
            contexts: Vec::new()
        }
    }
}

impl reactor::EventHandler<Event> for TimerEventHandler {
    type Handler = event::EventHandler;
    fn handle_event(&mut self, event: Event) {
            println!("timer event");
            let event: Event = event.into();
            for i in self.contexts.iter_mut() {
                //i.borrow_mut().handle_event(event.clone());
            }
    }
    fn handler(&self) -> Self::Handler {
        event::EventHandler::Timer
    }
}
