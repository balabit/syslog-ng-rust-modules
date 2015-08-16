use std::cell::RefCell;
use std::rc::Rc;

use dispatcher::request::{InternalRequest, RequestHandler};
use dispatcher::response::ResponseHandler;
use dispatcher::Response;
use context::event::EventHandler;
use reactor;

pub struct TimerEventHandler {
    contexts: Vec<Rc<RefCell<Box<EventHandler<InternalRequest>>>>>,
    response_handler: Rc<RefCell<Box<ResponseHandler<Response>>>>,
}

impl TimerEventHandler {
    pub fn new(response_handler: Rc<RefCell<Box<ResponseHandler<Response>>>>) -> TimerEventHandler {
        TimerEventHandler {
            contexts: Vec::new(),
            response_handler: response_handler
        }
    }

    pub fn register_handler(&mut self, handler: Rc<RefCell<Box<EventHandler<InternalRequest>>>>) {
        self.contexts.push(handler);
    }
}

impl reactor::EventHandler<InternalRequest> for TimerEventHandler {
    type Handler = RequestHandler;
    fn handle_event(&mut self, event: InternalRequest) {
            println!("timer event");
            let event: InternalRequest = event.into();
            for i in self.contexts.iter_mut() {
                if let Some(result) = i.borrow_mut().handle_event(event.clone()) {
                    for i in result.into_iter() {
                        self.response_handler.borrow_mut().handle_response(i.into());
                    }
                }
            }
    }
    fn handler(&self) -> Self::Handler {
        RequestHandler::Timer
    }
}
