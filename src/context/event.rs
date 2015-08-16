use std::rc::Rc;

use action::ExecResult;
use message::Message;
use timer::TimerEvent;

pub trait EventHandler<T> {
    fn handle_event(&mut self, T) -> Option<Vec<ExecResult>>;
    fn handlers(&self) -> &[String];
}
