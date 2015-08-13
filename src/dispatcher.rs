use std::sync::mpsc::{Receiver, Sender};
use std::rc::Rc;

use action::ExecResult;
use super::{config, Condition, Context, Event, Message, TimerEvent};
use reactor;

#[derive(Debug)]
pub enum Request {
    Event(Event),
    Exit
}

#[derive(Debug)]
pub enum Response {
    Event(ExecResult),
    Exit
}

pub struct Dispatcher {
    contexts: Vec<Context>,
    output_channel: Sender<Response>,
    exits_received: u32
}

impl Dispatcher {
    pub fn new(contexts: Vec<config::Context>, action_output_channel: Sender<Response>) -> Dispatcher {
        let contexts = contexts.into_iter().map(|ctx| Context::from(ctx)).collect::<Vec<Context>>();
        Dispatcher {
            contexts: contexts,
            output_channel: action_output_channel,
            exits_received: 0
        }
    }

    pub fn start_loop(&mut self, channel: Receiver<Request>) {
        for i in channel.iter() {
            match i {
                Request::Event(event) => self.dispatch(event),
                Request::Exit => {
                    if self.on_exit() {
                        break;
                    }
                }
            }
        }
    }

    fn on_exit(&mut self) -> bool {
        self.exits_received += 1;
        let _ = self.output_channel.send(Response::Exit);
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

mod handlers {
    pub mod exit {
        use dispatcher::Request;
        use condition::Condition;
        use reactor::EventHandler;

        pub struct ExitHandler{
            condition: Condition,
            stops: u32
        }

        impl ExitHandler {
            pub fn new(condition: Condition) -> ExitHandler {
                ExitHandler {
                    condition: condition,
                    stops: 0
                }
            }
        }

        impl EventHandler<Request> for ExitHandler {
            fn handle_event(&mut self, event: Request) {
                if let Request::Exit = event {
                    self.stops += 1;

                    if self.stops >= 2 {
                        self.condition.activate();
                    }
                } else {
                    unreachable!("An ExitHandler should only receive Exit events");
                }
            }
        }
    }

    pub mod event {
        use dispatcher::Request;
        use reactor;

        pub struct EventHandler;

        impl reactor::EventHandler<Request> for EventHandler {
            fn handle_event(&mut self, event: Request) {
                if let Request::Event(_) = event {
                    println!("Event recvd");
                } else {
                    unreachable!("An EventHandler should only receive Event events");
                }
            }
        }
    }
}
