use dispatcher::ResponseHandle;
use Response;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc;
use dispatcher::request::Request;

use reactor::EventHandler;
use action::Alert;

pub struct MessageEventHandler {
    pub responses: Rc<RefCell<Vec<Alert>>>,
}

impl EventHandler<Response, mpsc::Sender<Request>> for MessageEventHandler {
    fn handle_event(&mut self, event: Response, _: &mut mpsc::Sender<Request>) {
        if let Response::Alert(event) = event {
            self.responses.borrow_mut().push(event);
        }
    }
    fn handle(&self) -> ResponseHandle {
        ResponseHandle::Alert
    }
}
