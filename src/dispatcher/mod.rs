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
