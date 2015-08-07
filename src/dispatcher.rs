use std::sync::mpsc::Sender;
use std::rc::Rc;

use action::ActionCommand;
use super::{config, Context, Event};

pub struct Dispatcher {
    contexts: Vec<Context>,
    output_channel: Sender<ActionCommand>
}

impl Dispatcher {
    pub fn new(contexts: Vec<config::Context>, action_output_channel: Sender<ActionCommand>) -> Dispatcher {
        let contexts = contexts.into_iter().map(|ctx| Context::from(ctx)).collect::<Vec<Context>>();
        Dispatcher {
            contexts: contexts,
            output_channel: action_output_channel
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
