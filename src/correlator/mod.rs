use serde_json::from_str;
use std::collections::HashMap;
use std::io::Read;
use std::fs::File;
use std::sync::mpsc;
use std::thread;
use std::result::Result;
use std::time::Duration;
use std::sync::Arc;

use {Message, Response};
use config::ContextConfig;
use condition::Condition;
use context::base::BaseContextBuilder;
use context::{Context, ContextMap};
use context::linear::LinearContext;
use context::map::MapContext;
use dispatcher::request::Request;
use dispatcher::reactor::RequestReactor;
use dispatcher::{ResponseSender, ResponseHandle};
use dispatcher::demux::Demultiplexer;
use dispatcher::handlers;
pub use self::error::Error;
use reactor::{Event, Reactor, EventHandler};
use timer::Timer;

use self::exit_handler::ExitHandler;

const TIMER_STEP_MS: u64 = 100;

pub mod error;
mod exit_handler;
#[cfg(test)]
mod test;

pub struct Correlator {
    dispatcher_input_channel: mpsc::Sender<Request>,
    dispatcher_output_channel: mpsc::Receiver<Response>,
    dispatcher_thread_handle: thread::JoinHandle<()>,
    handlers: HashMap<ResponseHandle, Box<EventHandler<Response, mpsc::Sender<Request>>>>,
}

fn create_context(config_context: ContextConfig) -> Context {
    let ContextConfig{name, uuid, conditions, context_id, actions} = config_context;

    let base = BaseContextBuilder::new(uuid, conditions);
    let base = base.name(name);
    let base = base.actions(actions);
    let base = base.build();
    if let Some(context_id) = context_id {
        Context::Map(MapContext::new(base, context_id))
    } else {
        Context::Linear(LinearContext::from(base))
    }
}

fn create_context_map(contexts: Vec<ContextConfig>) -> ContextMap {
    let mut context_map = ContextMap::new();
    for i in contexts.into_iter() {
        let context: Context = create_context(i);
        context_map.insert(context);
    }
    context_map
}

impl Correlator {
    pub fn from_path(path: &str) -> Result<Correlator, Error> {
        let mut file = try!(File::open(path));
        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));
        let contexts = try!(from_str::<Vec<ContextConfig>>(&buffer));
        trace!("Correlator: loading contexts from file; len={}",
               contexts.len());
        Ok(Correlator::new(contexts))
    }

    pub fn new(contexts: Vec<ContextConfig>) -> Correlator {
        let (dispatcher_input_channel, rx) = mpsc::channel();
        let (dispatcher_output_channel_tx, dispatcher_output_channel_rx) = mpsc::channel();
        let _ = Timer::from_chan(Duration::from_millis(TIMER_STEP_MS),
                                 dispatcher_input_channel.clone());

        let handle = thread::spawn(move || {
            let exit_condition = Condition::new(false);
            let dmux = Demultiplexer::new(rx, exit_condition.clone());
            let response_sender = Box::new(ResponseSender::new(dispatcher_output_channel_tx));

            let exit_handler = Box::new(handlers::exit::ExitEventHandler::new(exit_condition));
            let timer_event_handler = Box::new(handlers::timer::TimerEventHandler::new());
            let message_event_handler = Box::new(handlers::message::MessageEventHandler::new());

            let context_map = create_context_map(contexts);
            let mut reactor = RequestReactor::new(dmux, context_map, response_sender);
            reactor.register_handler(exit_handler);
            reactor.register_handler(timer_event_handler);
            reactor.register_handler(message_event_handler);
            reactor.handle_events();
            trace!("Correlator: dispatcher thread exited");
        });

        Correlator {
            dispatcher_input_channel: dispatcher_input_channel,
            dispatcher_output_channel: dispatcher_output_channel_rx,
            dispatcher_thread_handle: handle,
            handlers: HashMap::new(),
        }
    }

    pub fn register_handler(&mut self,
                            handler: Box<EventHandler<Response, mpsc::Sender<Request>>>) {
        self.handlers.insert(handler.handle(), handler);
    }

    pub fn push_message(&mut self, message: Message) -> Result<(), mpsc::SendError<Request>> {
        self.handle_events();
        self.dispatcher_input_channel.send(Request::Message(Arc::new(message)))
    }

    fn handle_event(&mut self, event: Response) {
        if let Some(handler) = self.handlers.get_mut(&event.handle()) {
            handler.handle_event(event, &mut self.dispatcher_input_channel);
        } else {
            trace!("no event handler found for handling a Response");
        }
    }

    pub fn handle_events(&mut self) {
        for i in self.dispatcher_output_channel.try_recv() {
            self.handle_event(i);
        }
    }

    pub fn stop(mut self) -> thread::Result<()> {
        self.handle_events();
        self.stop_dispatcher();
        self.dispatcher_thread_handle.join()
    }

    fn stop_dispatcher(&mut self) {
        let exit_condition = Condition::new(false);
        let exit_handler = Box::new(ExitHandler::new(exit_condition.clone()));
        self.register_handler(exit_handler);
        let _ = self.dispatcher_input_channel.send(Request::Exit);
        while !exit_condition.is_active() {
            if let Ok(event) = self.dispatcher_output_channel.recv() {
                self.handle_event(event);
            }
        }
    }
}
