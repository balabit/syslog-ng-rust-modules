// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::time::Duration;
use std::sync::Arc;
use std::collections::VecDeque;

use Alert;
use context::ContextMap;
use dispatcher::request::Request;
use dispatcher::handlers::timer::TimerEventHandler;
use dispatcher::handlers::message::MessageEventHandler;
use reactor::{EventHandler, SharedData};
use timer::TimerEvent;
use Event;
use Template;

pub use self::error::Error;
pub use self::factory::CorrelatorFactory;

mod error;
mod factory;
#[cfg(test)]
mod test;

pub struct Correlator<E, T> where E: 'static + Event, T: 'static + Template<Event=E> {
    pub context_map: ContextMap<E, T>,
    pub responses: VecDeque<Alert<E>>,
    message_event_handler: MessageEventHandler,
    timer_event_handler: TimerEventHandler,
}

impl<E, T> Correlator<E, T> where E: Event, T: 'static + Template<Event=E> {
    pub fn new(context_map: ContextMap<E, T>) -> Correlator<E, T> {
        Correlator {
            context_map: context_map,
            responses: VecDeque::new(),
            message_event_handler: MessageEventHandler::default(),
            timer_event_handler: TimerEventHandler::default(),
        }
    }

    pub fn push_message(&mut self, message: E) {
        let mut shared_data = SharedData::new(&mut self.context_map, &mut self.responses);
        self.message_event_handler.handle_event(Request::Message(Arc::new(message)), &mut shared_data);
    }

    pub fn elapse_time(&mut self, span: Duration) {
        let mut shared_data = SharedData::new(&mut self.context_map, &mut self.responses);
        self.timer_event_handler.handle_event(Request::Timer(TimerEvent(span)), &mut shared_data);
    }
}
