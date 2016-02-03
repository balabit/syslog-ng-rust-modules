use std::sync::mpsc::Sender;

use action::Alert;
use reactor::Event;
use self::response::ResponseSender;

pub mod demux;
pub mod handlers;
pub mod response;
pub mod request;
pub mod reactor;

#[derive(Debug, Clone)]
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

impl ResponseSender for Sender<Response> {
    fn send_response(&mut self, response: Response) {
        let _ = self.send(response);
    }
}
