use std::sync::mpsc::{Receiver, Sender};
use std::rc::Rc;

use super::{config, Command, CommandResult, Context, Event, Message, TimerEvent};

pub struct Dispatcher {
    contexts: Vec<Context>,
    output_channel: Sender<CommandResult>,
    exits_received: u32
}

impl Dispatcher {
    pub fn new(contexts: Vec<config::Context>, action_output_channel: Sender<CommandResult>) -> Dispatcher {
        let contexts = contexts.into_iter().map(|ctx| Context::from(ctx)).collect::<Vec<Context>>();
        Dispatcher {
            contexts: contexts,
            output_channel: action_output_channel,
            exits_received: 0
        }
    }

    pub fn start_loop(&mut self, channel: Receiver<Command>) {
        for i in channel.iter() {
            match i {
                Command::Dispatch(event) => self.dispatch(event),
                Command::Exit => {
                    if self.on_exit() {
                        break;
                    }
                }
            }
        }
    }

    fn on_exit(&mut self) -> bool {
        self.exits_received += 1;
        let _ = self.output_channel.send(CommandResult::Exit);
        self.exits_received >= 2
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
                    let _ = self.output_channel.send(i.into());
                }
            }
        }
    }

    fn on_timer(&mut self, event: &TimerEvent) {
        for context in self.contexts.iter_mut() {
            if let Some(result) = context.on_timer(event) {
                for i in result.into_iter() {
                    let _ = self.output_channel.send(i.into());
                }
            }
        }
    }
}
