use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::result::Result;

use {action, config, context, Message, MiliSec, Response};
use condition::Condition;
use context::base;
use context::{Context};
use context::linear::LinearContext;
use dispatcher::request::{InternalRequest, Request};
use dispatcher::reactor::RequestReactor;
use dispatcher::{ResponseSender, ResponseHandler};
use dispatcher::response;
use dispatcher::demux::Demultiplexer;
use dispatcher::handlers;
use reactor::{Event, EventHandler, Reactor};
use timer::Timer;

const TIMER_STEP: MiliSec = 100;

pub struct Correlator {
    dispatcher_input_channel: mpsc::Sender<Request<Message>>,
    dispatcher_output_channel: mpsc::Receiver<Response>,
    dispatcher_thread_handle: thread::JoinHandle<()>,
    handlers: HashMap<ResponseHandler, Box<EventHandler<Response>>>
}

fn create_context(config_context: config::Context, response_sender: Rc<RefCell<Box<response::ResponseSender<Response>>>>) -> Context {
    let config::Context{name, uuid, conditions, actions} = config_context;
    let mut boxed_actions = Vec::new();

    for i in actions.into_iter() {
        let action = action::from_config(i, response_sender.clone());
        boxed_actions.push(action);
    }
    let base = base::Builder::new(uuid, conditions);
    let base = base.name(name);
    let base = base.actions(boxed_actions);
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
                let context: context::Context = create_context(i, response_handler.clone());
                let event_handler: Box<context::event::EventHandler<InternalRequest>> = context.into();
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
            handlers: HashMap::new()
        }
    }

    pub fn register_handler(&mut self, handler: Box<EventHandler<Response>>) {
        self.handlers.insert(handler.handler(), handler);
    }

    pub fn push_message(&mut self, message: Message) -> Result<(), mpsc::SendError<Request<Message>>> {
        self.consume_results();
        self.dispatcher_input_channel.send(Request::Message(message))
    }

    fn handle_event(&mut self, event: Response) {
        if let Some(handler) = self.handlers.get_mut(&event.handler()) {
            handler.handle_event(event);
        } else {
            println!("no event handler found for handling a Response");
        }
    }

    fn consume_results(&mut self) {
        for i in self.dispatcher_output_channel.try_recv() {
            self.handle_event(i);
        }
    }

    pub fn stop(mut self) -> thread::Result<()> {
        self.consume_results();
        self.stop_dispatcher();
        self.dispatcher_thread_handle.join()
    }

    fn stop_dispatcher(&mut self) {
        let exit_condition = Condition::new(false);
        let exit_handler = Box::new(ExitHandler::new(exit_condition.clone(), self.dispatcher_input_channel.clone()));
        self.register_handler(exit_handler);
        let _ = self.dispatcher_input_channel.send(Request::Exit);
        while !exit_condition.is_active() {
            if let Ok(event) = self.dispatcher_output_channel.recv() {
                self.handle_event(event);
            }
        }
    }
}

struct ExitHandler {
    channel: mpsc::Sender<Request<Message>>,
    exits_received: u32,
    condition: Condition
}

impl ExitHandler {
    pub fn new(condition: Condition, channel: mpsc::Sender<Request<Message>>) -> ExitHandler {
        ExitHandler {
            channel: channel,
            exits_received: 0,
            condition: condition
        }
    }
}

impl EventHandler<Response> for ExitHandler {
    fn handle_event(&mut self, event: Response) {
        if let Response::Exit = event {
            self.exits_received +=1;
            let _ = self.channel.send(Request::Exit);

            if self.exits_received >= 1 {
                self.condition.activate()
            }
        }
    }
    fn handler(&self) -> ResponseHandler {
        ResponseHandler::Exit
    }
}
