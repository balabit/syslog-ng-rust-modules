use std::sync::mpsc::Sender;

use super::Response;

pub trait ResponseSender {
    fn send_response(&mut self, response: Response);
}

impl ResponseSender for Sender<Response> {
    fn send_response(&mut self, response: Response) {
        let _ = self.send(response);
    }
}

#[derive(Clone)]
pub struct MockResponseSender(pub Vec<Response>);

impl MockResponseSender {
    pub fn new() -> MockResponseSender {
        MockResponseSender(Vec::new())
    }
}

impl ResponseSender for MockResponseSender {
    fn send_response(&mut self, response: Response) {
        self.0.push(response);
    }
}
