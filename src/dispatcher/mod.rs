use std::sync::mpsc::Sender;

pub mod demux;
pub mod handlers;
pub mod response;
pub mod request;
pub mod reactor;

#[derive(Debug)]
pub enum Response {
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

impl self::response::ResponseHandler<Response> for ResponseHandler {
    fn handle_response(&mut self, response: Response) {
        let _ = self.sender.send(response);
    }
}
