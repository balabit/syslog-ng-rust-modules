use std::cell::RefCell;
use std::rc::Rc;

use dispatcher::response::ResponseSender;
use dispatcher::Response;
use dispatcher::request::{Request, RequestHandler};
use condition::Condition;
use message::Message;
use reactor::EventHandler;

pub struct ExitEventHandler {
    condition: Condition,
    response_handler: Rc<RefCell<Box<ResponseSender<Response>>>>,
    stops: u32
}

impl ExitEventHandler {
    pub fn new(condition: Condition, response_handler: Rc<RefCell<Box<ResponseSender<Response>>>>) -> ExitEventHandler {
        ExitEventHandler {
            condition: condition,
            response_handler: response_handler,
            stops: 0
        }
    }
}

impl EventHandler<Request<Rc<Message>>> for ExitEventHandler {
    type Handler = RequestHandler;
    fn handle_event(&mut self, event: Request<Rc<Message>>) {
        if let Request::Exit = event {
            self.stops += 1;
            self.response_handler.borrow_mut().send_response(Response::Exit);

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
