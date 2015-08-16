use std::sync::mpsc::Sender;

use action::ExecResult;

pub mod demux;
pub mod handlers;
pub mod response;
pub mod request;
pub mod reactor;

#[derive(Debug)]
pub enum Response {
    Event(ExecResult),
    Exit
}

pub struct ResponseHandler {
    sender: Sender<Response>
}

impl ResponseHandler {
    pub fn new(sender: Sender<Response>) -> ResponseHandler {
        ResponseHandler {
            sender: sender
        }
    }
}

impl Into<Response> for ExecResult {
    fn into(self) -> Response {
        Response::Event(self)
    }
}

impl self::response::ResponseHandler<Response> for ResponseHandler {
    fn handle_response(&mut self, response: Response) {
        let _ = self.sender.send(response);
    }
}
