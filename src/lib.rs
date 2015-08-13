
#[macro_use]
extern crate maplit;
#[macro_use]
mod macros;

pub use action::Action;
pub use condition::Condition;
pub use conditions::Conditions;
pub use context::Context;
pub use correlator::Correlator;
pub use dispatcher::{Dispatcher, Response, Request};
pub use message::Message;
pub use timer::{Timer,
                TimerEvent};

pub mod conditions;
pub mod config;
pub mod action;
mod condition;
mod context;
mod correlator;
mod dispatcher;
pub mod message;
mod state;
mod reactor;
mod timer;

pub type MiliSec = u32;

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
