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
use context::ContextMap;
use dispatcher::request::Request;
use dispatcher::reactor::RequestReactor;
use dispatcher::ResponseHandle;
use dispatcher::demux::Demultiplexer;
use dispatcher::handlers::exit::ExitEventHandler;
use dispatcher::handlers::timer::TimerEventHandler;
use dispatcher::handlers::message::MessageEventHandler;
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
    dispatcher_thread_handle: thread::JoinHandle<ContextMap>,
    handlers: HashMap<ResponseHandle, Box<EventHandler<Response, mpsc::Sender<Request>>>>,
}

impl Correlator {
    pub fn from_path(path: &str) -> Result<Correlator, Error> {
        let mut file = try!(File::open(path));
        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));
        let contexts = try!(from_str::<Vec<ContextConfig>>(&buffer));
        trace!("Correlator: loading contexts from file; len={}",
               contexts.len());
        Ok(Correlator::new(ContextMap::from_configs(contexts)))
    }

    pub fn new(context_map: ContextMap) -> Correlator {
        let (dispatcher_input_channel, rx) = mpsc::channel();
        let (dispatcher_output_channel_tx, dispatcher_output_channel_rx) = mpsc::channel();
        Timer::from_chan(Duration::from_millis(TIMER_STEP_MS),
                         dispatcher_input_channel.clone());

        let handle = thread::spawn(move || {
            let dmux = Demultiplexer::new(rx);
            let response_sender = Box::new(dispatcher_output_channel_tx);

            let exit_handler = Box::new(ExitEventHandler::new());
            let timer_event_handler = Box::new(TimerEventHandler::new());
            let message_event_handler = Box::new(MessageEventHandler::new());

            let mut reactor = RequestReactor::new(dmux, context_map, response_sender);
            reactor.register_handler(exit_handler);
            reactor.register_handler(timer_event_handler);
            reactor.register_handler(message_event_handler);
            reactor.handle_events();
            trace!("Correlator: dispatcher thread exited");
            reactor.context_map
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

    pub fn stop(mut self) -> thread::Result<ContextMap> {
        self.handle_events();
        self.stop_dispatcher();
        self.dispatcher_thread_handle.join()
    }

    fn stop_dispatcher(&mut self) {
        let exit_handler = Box::new(ExitHandler::new());
        self.register_handler(exit_handler);
        let _ = self.dispatcher_input_channel.send(Request::Exit);
        while let Ok(event) = self.dispatcher_output_channel.recv() {
            self.handle_event(event);
        }
    }
}
