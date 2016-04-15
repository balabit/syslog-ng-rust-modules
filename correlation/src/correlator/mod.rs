// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;
use std::sync::Arc;

use Response;
use action::Alert;
use context::ContextMap;
use dispatcher::request::Request;
use dispatcher::reactor::RequestReactor;
use dispatcher::demux::Demultiplexer;
use dispatcher::handlers::exit::ExitEventHandler;
use dispatcher::handlers::timer::TimerEventHandler;
use dispatcher::handlers::message::MessageEventHandler;
use reactor::{Reactor, EventHandler};
use timer::Timer;
use Event;
use Template;

const TIMER_STEP_MS: u64 = 100;

pub use self::error::Error;
pub use self::factory::CorrelatorFactory;

mod error;
mod factory;
#[cfg(test)]
mod test;

pub trait AlertHandler<D, E> where E: Event {
    fn on_alert(&mut self, alert: Alert<E>, channel: &mut Sender<Request<E>>, extra_data: &mut D);
}

pub struct Correlator<T, E, TPL> where E: 'static + Event, TPL: 'static + Template<Event=E> {
    dispatcher_input_channel: mpsc::Sender<Request<E>>,
    dispatcher_output_channel: mpsc::Receiver<Response<E>>,
    dispatcher_thread_handle: thread::JoinHandle<ContextMap<E, TPL>>,
    alert_handler: Option<Box<AlertHandler<T, E>>>
}

impl<T, E, TPL> Correlator<T, E, TPL> where E: Event, TPL: 'static + Template<Event=E> {
    pub fn new(context_map: ContextMap<E, TPL>) -> Correlator<T, E, TPL> {
        let (dispatcher_input_channel, rx) = mpsc::channel();
        let (dispatcher_output_channel_tx, dispatcher_output_channel_rx) = mpsc::channel();
        Timer::from_chan(Duration::from_millis(TIMER_STEP_MS),
                         dispatcher_input_channel.clone());

        let handle = thread::spawn(move || {
            let dmux = Demultiplexer::new(rx);
            let response_sender = Box::new(dispatcher_output_channel_tx);

            let exit_handler = Box::new(ExitEventHandler::default());
            let timer_event_handler = Box::new(TimerEventHandler::default());
            let message_event_handler = Box::new(MessageEventHandler::default());

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
            alert_handler: None
        }
    }

    pub fn set_alert_handler(&mut self,
                            handler: Option<Box<AlertHandler<T, E>>>) {
        self.alert_handler = handler;
    }

    pub fn push_message(&mut self, message: E) -> Result<(), mpsc::SendError<Request<E>>> {
        self.dispatcher_input_channel.send(Request::Message(Arc::new(message)))
    }

    fn handle_event(&mut self, event: Response<E>, external_handler_data: &mut T) {
        match event {
            Response::Exit => {
                let _ = self.dispatcher_input_channel.send(Request::Exit);
            },
            Response::Alert(alert) => {
                if let Some(handler) = self.alert_handler.as_mut() {
                    handler.on_alert(alert, &mut self.dispatcher_input_channel, external_handler_data);
                } else {
                    trace!("No Alert handler is registereted in Correlator but an alert is received");
                }
            }
        }
    }

    pub fn handle_events(&mut self, external_handler_data: &mut T) {
        for i in self.dispatcher_output_channel.try_recv() {
            self.handle_event(i, external_handler_data);
        }
    }

    pub fn stop(mut self, external_handler_data: &mut T) -> thread::Result<ContextMap<E, TPL>> {
        self.handle_events(external_handler_data);
        self.stop_dispatcher(external_handler_data);
        self.dispatcher_thread_handle.join()
    }

    fn stop_dispatcher(&mut self, external_handler_data: &mut T) {
        let _ = self.dispatcher_input_channel.send(Request::Exit);
        while let Ok(event) = self.dispatcher_output_channel.recv() {
            self.handle_event(event, external_handler_data);
        }
    }
}
