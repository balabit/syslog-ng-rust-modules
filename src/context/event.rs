use action::ExecResult;

pub trait EventHandler<T> {
    fn handle_event(&mut self, T) -> Option<Vec<ExecResult>>;
    fn handlers(&self) -> &[String];
}
