use std::sync::mpsc::Sender;
use std::rc::Rc;

use action::ActionCommand;
use super::{config, Context, Event, Message, TimerEvent};

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
        let mut commands = Vec::new();
        match event {
            Event::Message(event) => {
                let event = Rc::new(event);
                self.on_message(&mut commands, event);
            },
            Event::Timer(ref event) => {
                self.on_timer(&mut commands, event);
            }
        };

        for i in commands.into_iter() {
            self.output_channel.send(i);
        }
    }

    fn on_message(&mut self, commands: &mut Vec<ActionCommand>, event: Rc<Message>) {
        for context in self.contexts.iter_mut() {
            if let Some(result) = context.on_message(event.clone()) {
                for i in result.into_iter() {
                    commands.push(i);
                }
            }
        }
    }

    fn on_timer(&mut self, commands: &mut Vec<ActionCommand>, event: &TimerEvent) {
        for context in self.contexts.iter_mut() {
            if let Some(result) = context.on_timer(event) {
                for i in result.into_iter() {
                    commands.push(i);
                }
            }
        }
    }
}
