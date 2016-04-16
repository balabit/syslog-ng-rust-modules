// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use uuid::Uuid;
use std::collections::VecDeque;
use std::time::Duration;

use Alert;
use state::State;
use context::base::BaseContext;
use Event;
use Template;

pub struct LinearContext<E, T> where E: Event, T: Template<Event=E> {
    base: BaseContext<E, T>,
    state: State<E>,
}

impl<E, T> LinearContext<E, T> where E: Event, T: Template<Event=E> {
    pub fn new(base: BaseContext<E, T>) -> LinearContext<E, T> {
        LinearContext {
            base: base,
            state: State::new(),
        }
    }

    pub fn on_timer(&mut self, event: &Duration, responder: &mut VecDeque<Alert<E>>) {
        self.base.on_timer(event, &mut self.state, responder);
    }

    pub fn on_message(&mut self, event: E, responder: &mut VecDeque<Alert<E>>) {
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
