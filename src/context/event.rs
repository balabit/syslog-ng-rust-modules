use action::ExecResult;
use message::PatternId;

pub trait EventHandler<T> {
    fn handle_event(&mut self, T) -> Option<Vec<ExecResult>>;
    fn handlers(&self) -> &[PatternId];
}
