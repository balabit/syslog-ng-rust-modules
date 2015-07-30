use std::rc::Rc;

use super::{context, Context, Event};

pub struct Dispatcher {
    contexts: Vec<Context>,
}

impl Dispatcher {
    pub fn new(contexts: Vec<Context>) -> Dispatcher {
        Dispatcher {
            contexts: contexts,
        }
    }

    pub fn dispatch(&mut self, event: Event) {
        match event {
            Event::Message(event) => {
                let event = Rc::new(event);
                for context in self.contexts.iter_mut() {
                    context.on_message(event);
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
