use dispatcher::ResponseHandler;
use Response;

use std::rc::Rc;
use std::cell::RefCell;

use reactor::EventHandler;
use action::MessageResponse;

pub struct MessageEventHandler {
    pub responses: Rc<RefCell<Vec<MessageResponse>>>,
}

impl EventHandler<Response, ()> for MessageEventHandler {
    fn handle_event(&mut self, event: Response, _: &mut ()) {
        if let Response::Message(event) = event {
            self.responses.borrow_mut().push(event);
        }
    }
    fn handler(&self) -> ResponseHandler {
        ResponseHandler::Message
    }
}
