// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::sync::Arc;

use Message;
use timer::TimerEvent;
use std::time::Duration;

#[derive(Debug)]
pub struct State {
    elapsed_time: Duration,
    elapsed_time_since_last_message: Duration,
    messages: Vec<Arc<Message>>,
    opened: bool,
}

impl State {
    pub fn new() -> State {
        State::with_messages(Vec::new())
    }

    pub fn with_messages(messages: Vec<Arc<Message>>) -> State {
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

    pub fn messages(&self) -> &[Arc<Message>] {
        &self.messages
    }

    pub fn add_message(&mut self, message: Arc<Message>) {
        self.messages.push(message);
        self.elapsed_time_since_last_message = Duration::from_secs(0);
    }

    pub fn update_timers(&mut self, event: &TimerEvent) {
        let delta = event.0;
        self.elapsed_time = self.elapsed_time + delta;
        self.elapsed_time_since_last_message = self.elapsed_time_since_last_message + delta;
    }

    fn reset(&mut self) {
        self.elapsed_time = Duration::from_secs(0);
        self.elapsed_time_since_last_message = Duration::from_secs(0);
        self.messages.clear();
        self.opened = false;
    }
}
