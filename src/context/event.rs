pub trait EventHandler<T> {
    fn handle_event(&mut self, T);
    fn handlers(&self) -> &[String];
}
