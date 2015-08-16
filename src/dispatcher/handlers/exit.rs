use std::rc::Rc;

use dispatcher::request::{Request, RequestHandler};
use condition::Condition;
use message::Message;
use reactor::EventHandler;

pub struct ExitEventHandler {
    condition: Condition,
    stops: u32
}

impl ExitEventHandler {
    pub fn new(condition: Condition) -> ExitEventHandler {
        ExitEventHandler {
            condition: condition,
            stops: 0
        }
    }
}

impl EventHandler<Request<Rc<Message>>> for ExitEventHandler {
    type Handler = RequestHandler;
    fn handle_event(&mut self, event: Request<Rc<Message>>) {
        if let Request::Exit = event {
            self.stops += 1;

            if self.stops >= 2 {
                self.condition.activate();
            }
        } else {
            unreachable!("An ExitEventHandler should only receive Exit events");
        }
    }
    fn handler(&self) -> Self::Handler {
        RequestHandler::Exit
    }
}
