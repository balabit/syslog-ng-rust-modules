use super::Response;

pub trait ResponseSender {
    fn send_response(&self, response: Response);
    fn boxed_clone(&self) -> Box<ResponseSender>;
}
