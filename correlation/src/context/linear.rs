// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use uuid::Uuid;
use std::sync::Arc;

use state::State;
use timer::TimerEvent;
use dispatcher::request::Request;
use dispatcher::response::ResponseSender;
use context::base::BaseContext;
use Event;

pub struct LinearContext<E: Event> {
    base: BaseContext,
    state: State<E>,
}

impl<E: Event> LinearContext<E> {
    pub fn new(base: BaseContext) -> LinearContext<E> {
        LinearContext {
            base: base,
            state: State::new(),
        }
    }

    pub fn on_event(&mut self, event: Request<E>, responder: &mut ResponseSender<E>) {
        trace!("LinearContext: received event");
        match event {
            Request::Timer(event) => self.on_timer(&event, responder),
            Request::Message(message) => self.on_message(message, responder),
            _ => {}
        }
    }

    pub fn on_timer(&mut self, event: &TimerEvent, responder: &mut ResponseSender<E>) {
        self.base.on_timer(event, &mut self.state, responder);
    }

    pub fn on_message(&mut self, event: Arc<E>, responder: &mut ResponseSender<E>) {
        self.base.on_message(event, &mut self.state, responder);
    }

    #[allow(dead_code)]
    pub fn is_open(&self) -> bool {
        self.state.is_open()
    }

    pub fn patterns(&self) -> &[String] {
        &self.base.patterns
    }

    #[allow(dead_code)]
    pub fn uuid(&self) -> &Uuid {
        self.base.uuid()
    }
}
