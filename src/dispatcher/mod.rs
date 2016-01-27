use std::sync::mpsc::Sender;
use std::rc::Rc;
use std::cell::RefCell;

use action::MessageResponse;
use reactor::Event;

pub mod demux;
pub mod handlers;
pub mod response;
pub mod request;
pub mod reactor;

#[derive(Debug)]
pub enum Response {
    Exit,
    Message(MessageResponse),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ResponseHandler {
    Exit,
    Message,
}

impl Event for Response {
    type Handler = ResponseHandler;
    fn handler(&self) -> Self::Handler {
        match *self {
            Response::Exit => ResponseHandler::Exit,
            Response::Message(_) => ResponseHandler::Message,
        }
    }
}

#[derive(Clone)]
pub struct ResponseSender {
    sender: Sender<Response>,
}

impl ResponseSender {
    pub fn new(sender: Sender<Response>) -> ResponseSender {
        ResponseSender { sender: sender }
    }
}

impl self::response::ResponseSender<Response> for ResponseSender {
    fn send_response(&mut self, response: Response) {
        let _ = self.sender.send(response);
    }

    fn boxed_clone(&self) -> Box<self::response::ResponseSender<Response>> {
        Box::new(self.clone())
    }
}
