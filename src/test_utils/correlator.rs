use dispatcher::ResponseHandler;
use Response;
use Message;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc;
use dispatcher::request::Request;

use reactor::EventHandler;
use action::MessageResponse;

pub struct MessageEventHandler {
    pub responses: Rc<RefCell<Vec<MessageResponse>>>,
}

impl EventHandler<Response, mpsc::Sender<Request<Message>>> for MessageEventHandler {
    fn handle_event(&mut self, event: Response, _: &mut mpsc::Sender<Request<Message>>) {
        if let Response::Message(event) = event {
            self.responses.borrow_mut().push(event);
        }
    }
    fn handler(&self) -> ResponseHandler {
        ResponseHandler::Message
    }
}
