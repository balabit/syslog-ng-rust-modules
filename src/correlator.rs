use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::result::Result;

use {action, config, context, Message, MiliSec, Response, Timer};
use context::base;
use context::{Context};
use context::linear::LinearContext;
use condition::Condition;
use context::event::EventHandler;
use dispatcher::request::{InternalRequest, Request};
use dispatcher::reactor::RequestReactor;
use dispatcher::ResponseSender;
use dispatcher::response;
use dispatcher::demux::Demultiplexer;
use dispatcher::handlers;
use reactor::Reactor;

const TIMER_STEP: MiliSec = 100;

pub struct Correlator {
    dispatcher_input_channel: mpsc::Sender<Request<Message>>,
    dispatcher_output_channel: mpsc::Receiver<Response>,
    dispatcher_thread_handle: thread::JoinHandle<()>,
    exits_received: u32
}

fn create_context(config_context: config::Context, response_sender: Rc<RefCell<Box<response::ResponseSender<Response>>>>) -> Context {
    let config::Context{name, uuid, conditions, actions} = config_context;
    let mut boxed_actions = Vec::new();

    for i in actions.into_iter() {
        let action = action::from_config(i, response_sender.clone());
        boxed_actions.push(action);
    }
    let mut base = base::Builder::new(uuid, conditions);
    let mut base = base.name(name);
    let mut base = base.actions(boxed_actions);
    let base = base.build();
    Context::Linear(LinearContext::from(base))
}

impl Correlator {
    pub fn new(contexts: Vec<config::Context>) -> Correlator {
        let (dispatcher_input_channel, rx) = mpsc::channel();
        let (dispatcher_output_channel_tx, dispatcher_output_channel_rx) = mpsc::channel();
        let _ = Timer::from_chan(TIMER_STEP, dispatcher_input_channel.clone());

        let handle = thread::spawn(move || {
            let dmux = Demultiplexer::new(rx);
            let exit_condition = Condition::new(false);
            let mut reactor = RequestReactor::new(dmux, exit_condition.clone());
            let response_handler = Box::new(ResponseSender::new(dispatcher_output_channel_tx)) as Box<response::ResponseSender<Response>>;
            let response_handler = Rc::new(RefCell::new(response_handler));

            let exit_handler = Box::new(handlers::exit::ExitEventHandler::new(exit_condition, response_handler.clone()));
            let mut timer_event_handler = Box::new(handlers::timer::TimerEventHandler::new());
            let mut message_event_handler = Box::new(handlers::message::MessageEventHandler::new());

            let mut event_handlers = Vec::new();
            for i in contexts.into_iter() {
                let mut context: context::Context = create_context(i, response_handler.clone());
                match context {
                    Context::Linear(ref mut context) => {
                        for action in context.actions_mut() {
                            action.set_response_sender(response_handler.clone());
                        }
                    },
                    Context::Map(ref mut context) => {
                        for action in context.actions_mut() {
                            action.set_response_sender(response_handler.clone());
                        }
                    }
                }

                let event_handler: Box<EventHandler<InternalRequest>> = context.into();
                let handler = Rc::new(RefCell::new(event_handler));
                event_handlers.push(handler);
            }

            for i in event_handlers {
                timer_event_handler.register_handler(i.clone());
                message_event_handler.register_handler(i.clone())
            }

            reactor.register_handler(exit_handler);
            reactor.register_handler(timer_event_handler);
            reactor.register_handler(message_event_handler);
            reactor.handle_events();
        });

        Correlator {
            dispatcher_input_channel: dispatcher_input_channel,
            dispatcher_output_channel: dispatcher_output_channel_rx,
            dispatcher_thread_handle: handle,
            exits_received: 0
        }
    }

    pub fn push_message(&mut self, message: Message) -> Result<(), mpsc::SendError<Request<Message>>> {
        self.consume_results();
        self.dispatcher_input_channel.send(Request::Message(message))
    }

    fn consume_results(&mut self) {
        for _ in self.dispatcher_output_channel.try_recv() {
        }
    }

    pub fn stop(mut self) -> thread::Result<()> {
        self.consume_results();
        self.stop_dispatcher();
        self.dispatcher_thread_handle.join()
    }

    fn stop_dispatcher(&mut self) {
        let _ = self.dispatcher_input_channel.send(Request::Exit);
        let _ = self.wait_for_dispatcher_to_exit();
    }

    fn wait_for_dispatcher_to_exit(&mut self) -> Result<(), ()> {
        loop {
            let value = self.dispatcher_output_channel.recv();
            match value {
                Ok(value) => {
                    try!(self.handle_command(value))
                },
                _ => {}
            }
        }
    }

    fn handle_command(&mut self, command: Response) -> Result<(), ()> {
        match command {
            Response::Exit => {
                if self.handle_exit_command() {
                    return Err(());
                }
            }
        }
        Ok(())
    }

    fn handle_exit_command(&mut self) -> bool {
        let _ = self.dispatcher_input_channel.send(Request::Exit);
        self.exits_received += 1;
        self.exits_received >= 1
    }
}
