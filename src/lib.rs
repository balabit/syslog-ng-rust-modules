
#[macro_use]
extern crate maplit;
#[macro_use]
mod macros;

use std::collections::BTreeMap;

pub use context::Context;
pub use dispatcher::Dispatcher;
pub use correlator::Correlator;
pub use timer::{Timer,
                TimerEvent};

mod context;
mod dispatcher;
mod correlator;
mod timer;

pub type Message = BTreeMap<String, String>;

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
