use std::sync::mpsc::Sender;

use super::Response;

pub trait ResponseSender {
    fn send_response(&mut self, response: Response);
    fn boxed_clone(&self) -> Box<ResponseSender>;
}

impl ResponseSender for Sender<Response> {
    fn send_response(&mut self, response: Response) {
        let _ = self.send(response);
    }
    fn boxed_clone(&self) -> Box<ResponseSender> {
        Box::new(self.clone())
    }
}
