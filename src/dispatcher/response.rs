pub trait ResponseSender<T> {
    fn send_response(&mut self, response: T);
}
