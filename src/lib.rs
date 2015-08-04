
#[macro_use]
extern crate maplit;
#[macro_use]
mod macros;
mod message;

use std::fmt::Debug;

pub use conditions::Conditions;
pub use context::Context;
pub use correlator::Correlator;
pub use dispatcher::Dispatcher;
pub use message::Message;
pub use timer::{Timer,
                TimerEvent};

pub mod conditions;
pub mod config;
mod context;
mod dispatcher;
mod correlator;
mod state;
mod timer;

pub type MiliSec = u32;

#[derive(Debug)]
pub enum Event {
    Timer(TimerEvent),
    Message(Message)
}

#[derive(Debug)]
pub enum Command {
    Dispatch(Event),
    Exit
}

pub trait Action: Debug {
    fn execute(&mut self, state: &state::State);
}
