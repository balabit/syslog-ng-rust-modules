use std::sync::mpsc::Sender;

use reactor::Event;

pub mod demux;
pub mod handlers;
pub mod response;
pub mod request;
pub mod reactor;

#[derive(Debug)]
pub enum Response {
    Exit
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ResponseHandler {
    Exit
}

impl Event for Response {
    type Handler = ResponseHandler;
    fn handler(&self) -> Self::Handler {
        match *self {
            Response::Exit => ResponseHandler::Exit
        }
    }
}

pub struct ResponseSender {
    sender: Sender<Response>
}

impl ResponseSender {
    pub fn new(sender: Sender<Response>) -> ResponseSender {
        ResponseSender {
            sender: sender
        }
    }
}

impl self::response::ResponseSender<Response> for ResponseSender {
    fn send_response(&mut self, response: Response) {
        let _ = self.sender.send(response);
    }
}
