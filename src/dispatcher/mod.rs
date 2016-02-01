use std::sync::mpsc::Sender;
use std::rc::Rc;
use std::cell::RefCell;

use action::Alert;
use reactor::Event;

pub mod demux;
pub mod handlers;
pub mod response;
pub mod request;
pub mod reactor;

#[derive(Debug)]
pub enum Response {
    Exit,
    Alert(Alert),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ResponseHandle {
    Exit,
    Alert,
}

impl Event for Response {
    type Handle = ResponseHandle;
    fn handle(&self) -> Self::Handle {
        match *self {
            Response::Exit => ResponseHandle::Exit,
            Response::Alert(_) => ResponseHandle::Alert,
        }
    }
}

#[derive(Clone)]
pub struct ResponseSender {
    sender: Rc<RefCell<Sender<Response>>>,
}

impl ResponseSender {
    pub fn new(sender: Sender<Response>) -> ResponseSender {
        ResponseSender { sender: Rc::new(RefCell::new(sender)) }
    }
}

impl self::response::ResponseSender for ResponseSender {
    fn send_response(&mut self, response: Response) {
        let sender = self.sender.borrow_mut();
        let _ = sender.send(response);
    }

    fn boxed_clone(&self) -> Box<self::response::ResponseSender> {
        Box::new(self.clone())
    }
}
