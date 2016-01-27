pub trait ResponseSender<T> {
    fn send_response(&self, response: T);
    fn boxed_clone(&self) -> Box<ResponseSender<T>>;
}
