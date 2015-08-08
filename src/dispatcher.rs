use std::sync::mpsc::{Receiver, Sender};
use std::rc::Rc;

use action::ExecResult;
use super::{config, Command, CommandResult, Context, Event, Message, TimerEvent};

pub struct Dispatcher {
    contexts: Vec<Context>,
    output_channel: Sender<CommandResult>
}

impl Dispatcher {
    pub fn new(contexts: Vec<config::Context>, action_output_channel: Sender<CommandResult>) -> Dispatcher {
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
                self.on_message(event);
            },
            Event::Timer(ref event) => {
                self.on_timer(event);
            }
        };
    }

    fn on_message(&mut self, event: Rc<Message>) {
        for context in self.contexts.iter_mut() {
            if let Some(result) = context.on_message(event.clone()) {
                for i in result.into_iter() {
                    let r = self.output_channel.send(i.into());
                    println!("{:?}", r);
                }
            }
        }
    }

    fn on_timer(&mut self, event: &TimerEvent) {
        for context in self.contexts.iter_mut() {
            if let Some(result) = context.on_timer(event) {
                for i in result.into_iter() {
                    let r = self.output_channel.send(i.into());
                    println!("{:?}", r);
                }
            }
        }
    }
}
