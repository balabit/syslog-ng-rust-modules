use std::rc::Rc;

use super::{config, context, Context, Event};

pub struct Dispatcher {
    contexts: Vec<Context>,
}

impl Dispatcher {
    pub fn new(contexts: Vec<config::Context>) -> Dispatcher {
        let contexts = contexts.into_iter().map(|ctx| Context::from(ctx)).collect::<Vec<Context>>();
        println!("{}", contexts.len());
        Dispatcher {
            contexts: contexts,
        }
    }

    pub fn dispatch(&mut self, event: Event) {
        match event {
            Event::Message(event) => {
                let event = Rc::new(event);
                for context in self.contexts.iter_mut() {
                    context.on_message(event.clone());
                }
            },
            Event::Timer(ref event) => {
                for context in self.contexts.iter_mut() {
                    context.on_timer(event);
                }
            }
        }
    }
}
