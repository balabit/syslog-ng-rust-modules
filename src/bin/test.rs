#[macro_use]
extern crate maplit;
extern crate correlation;
extern crate uuid;

use correlation::{config, conditions, Correlator};
use correlation::message::{Builder, PatternId};
use correlation::action::message;
use correlation::action::ActionHandlers;
use uuid::Uuid;
use std::thread;

struct Printer;

impl message::ActionHandler for Printer {
    fn handle(&mut self, _: message::ExecResult) {
        println!("I'm the message handler");
    }
}

#[allow(dead_code)]
fn main() {
    let uuid1 = "1b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let uuid2 = "2b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let uuid3 = "3b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let patterns = vec![
        PatternId::Name(uuid1.clone()),
        PatternId::Name(uuid2.clone()),
        PatternId::Name(uuid3.clone()),
    ];
    let condition = conditions::Builder::new(100).patterns(patterns)
                                                .first_opens(true)
                                                .last_closes(true)
                                                .build();
    let actions = vec![ message::Action::new().into() ];
    let contexts = vec!{
        config::ContextBuilder::new(Uuid::new_v4(), condition.clone()).actions(actions.clone()).build(),
        config::ContextBuilder::new(Uuid::new_v4(), condition.clone()).actions(actions.clone()).build(),
        config::ContextBuilder::new(Uuid::new_v4(), condition.clone()).actions(actions.clone()).build(),
    };
    let handlers = ActionHandlers::new(Box::new(Printer));
    let mut correlator = Correlator::new(contexts, handlers);
    let _ = correlator.push_message(Builder::new(&uuid1).build());
    thread::sleep_ms(20);
    let _ = correlator.push_message(Builder::new(&uuid2).build());
    thread::sleep_ms(80);
    let _ = correlator.push_message(Builder::new(&uuid3).build());
    let _ = correlator.stop();
}
