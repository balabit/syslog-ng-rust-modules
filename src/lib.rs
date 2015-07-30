
#[macro_use]
extern crate maplit;
#[macro_use]
mod macros;

use std::collections::BTreeMap;
use std::sync::mpsc;
use std::thread;

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


pub trait Observer {
    fn on_event(&mut self, event: &Event) -> bool {
        match *event {
            Event::Timer(ref event) => self.on_timer(event),
            Event::Message(ref event) => self.on_message(event),
        }
    }
    fn on_timer(&mut self, event: &TimerEvent) -> bool;
    fn on_message(&mut self, event: &Message) -> bool;
}
