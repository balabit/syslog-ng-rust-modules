// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use Event;
use std::time::Duration;

#[derive(Debug)]
pub struct State<E: Event> {
    elapsed_time: Duration,
    elapsed_time_since_last_message: Duration,
    messages: Vec<E>,
    opened: bool,
}

impl<E: Event> Default for State<E> {
    fn default() -> State<E> {
        State::with_messages(Vec::new())
    }
}

impl<E: Event> State<E> {
    pub fn new() -> State<E> {
        State::default()
    }

    pub fn with_messages(messages: Vec<E>) -> State<E> {
        State {
            elapsed_time: Duration::from_secs(0),
            elapsed_time_since_last_message: Duration::from_secs(0),
            messages: messages,
            opened: false,
        }
    }

    pub fn is_open(&self) -> bool {
        self.opened
    }

    pub fn open(&mut self) {
        self.opened = true;
    }

    pub fn close(&mut self) {
        self.reset();
    }

    pub fn elapsed_time(&self) -> Duration {
        self.elapsed_time
    }

    pub fn elapsed_time_since_last_message(&self) -> Duration {
        self.elapsed_time_since_last_message
    }

    pub fn messages(&self) -> &[E] {
        &self.messages
    }

    pub fn add_message(&mut self, message: E) {
        self.messages.push(message);
        self.elapsed_time_since_last_message = Duration::from_secs(0);
    }

    pub fn update_timers(&mut self, event: &Duration) {
        let delta = *event;
        self.elapsed_time +=  delta;
        self.elapsed_time_since_last_message += delta;
    }

    fn reset(&mut self) {
        self.elapsed_time = Duration::from_secs(0);
        self.elapsed_time_since_last_message = Duration::from_secs(0);
        self.messages.clear();
        self.opened = false;
    }
}
