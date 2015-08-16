pub trait ResponseHandler<T> {
    fn handle_response(&mut self, response: T);
}
