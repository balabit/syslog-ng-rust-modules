use super::Response;

pub trait ResponseSender {
    fn send_response(&mut self, response: Response);
}
