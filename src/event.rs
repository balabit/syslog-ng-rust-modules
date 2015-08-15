use message::Message;
use reactor;
use timer::{Timer,
                TimerEvent};
#[derive(Debug)]
pub enum Event {
    Timer(TimerEvent),
    Message(Message)
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum EventHandler {
    Timer,
    Message
}

impl reactor::Event for Event {
    type Handler = EventHandler;
    fn handler(&self) -> Self::Handler {
        match *self {
            Event::Message(_) => EventHandler::Message,
            Event::Timer(_) => EventHandler::Timer
        }
    }
}
