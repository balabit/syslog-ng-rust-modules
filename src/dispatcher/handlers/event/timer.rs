use std::cell::RefCell;
use std::rc::Rc;

use context::Context;
use dispatcher::request::{Request, RequestHandler};
use context::EventHandler;
use context;
use event;
use event::Event;
use reactor;
use TimerEvent;
use action::ExecResult;

pub struct TimerEventHandler {
    contexts: Vec<Rc<RefCell<Box<EventHandler<TimerEvent>>>>>
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
        if let event::Event::Timer(event) = event {
            println!("timer event");
            for i in self.contexts.iter_mut() {
                i.borrow_mut().handle_event(event);
            }
        } else {
            unreachable!("TimerEventHandler should only handle Timer events");
        }
    }
    fn handler(&self) -> Self::Handler {
        event::EventHandler::Timer
    }
}
