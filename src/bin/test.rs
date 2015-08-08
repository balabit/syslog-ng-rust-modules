#[macro_use]
extern crate maplit;
extern crate correlation;

use correlation::{config, conditions, Correlator, Message};
use correlation::action::message;
use correlation::action::ActionHandlers;
use std::thread;

struct Printer;

impl message::ActionHandler for Printer {
    fn handle(&mut self, _: message::ExecResult) {
        println!("I'm the message handler");
    }
}

#[allow(dead_code)]
fn main() {
    let patterns = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let condition = conditions::Builder::new(100).patterns(patterns)
                                                .first_opens(true)
                                                .last_closes(true)
                                                .build();
    let actions = vec![ message::Action::new().into() ];
    let contexts = vec!{
        config::ContextBuilder::new(condition.clone()).actions(actions.clone()).build(),
        config::ContextBuilder::new(condition.clone()).actions(actions.clone()).build(),
        config::ContextBuilder::new(condition.clone()).actions(actions.clone()).build(),
    };
    let handlers = ActionHandlers::new(Box::new(Printer));
    let mut correlator = Correlator::new(contexts, handlers);
    let _ = correlator.push_message(Message::new("1".to_string()));
    thread::sleep_ms(20);
    let _ = correlator.push_message(Message::new("2".to_string()));
    thread::sleep_ms(80);
    let _ = correlator.push_message(Message::new("3".to_string()));
    let _ = correlator.stop();
}
