use Response;
use dispatcher::response::ResponseSender;

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
