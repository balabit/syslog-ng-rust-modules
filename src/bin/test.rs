#[macro_use]
extern crate maplit;
extern crate correlation;

use correlation::{config, conditions, Correlator, Message};
use correlation::action::message::{MessageActionHandler, Command};
use correlation::action::ActionHandlers;
use std::thread;

struct Printer;

impl MessageActionHandler for Printer {
    fn handle(&mut self, command: Command) {
        println!("{:?}", &command);
    }
}

fn main() {
    let contexts = vec!{
        config::Context::new(conditions::Builder::new(100).build()),
        config::Context::new(conditions::Builder::new(100).build()),
        config::Context::new(conditions::Builder::new(100).build()),
    };
    let handlers = ActionHandlers::new(Box::new(Printer));
    let mut correlator = Correlator::new(contexts, handlers);
    let msg1 = Message::new("1".to_string());
    let _ = correlator.push_message(msg1);
    thread::sleep_ms(2000);
    let _ = correlator.stop();
}
