use std::rc::Rc;

use Message;
use MiliSec;
use timer::TimerEvent;

#[derive(Debug)]
pub struct State {
    elapsed_time: MiliSec,
    elapsed_time_since_last_message: MiliSec,
    messages: Vec<Rc<Message>>,
    opened: bool,
}

impl State {
    pub fn new() -> State {
        State {
            elapsed_time: 0,
            elapsed_time_since_last_message: 0,
            messages: Vec::new(),
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

    pub fn elapsed_time(&self) -> MiliSec {
        self.elapsed_time
    }

    pub fn elapsed_time_since_last_message(&self) -> MiliSec {
        self.elapsed_time_since_last_message
    }

    pub fn messages(&self) -> &[Rc<Message>] {
        &self.messages
    }

    pub fn add_message(&mut self, message: Rc<Message>) {
        self.messages.push(message);
        self.elapsed_time_since_last_message = 0;
    }

    pub fn update_timers(&mut self, event: &TimerEvent) {
        let delta = event.0;
        self.elapsed_time += delta;
        self.elapsed_time_since_last_message += delta;
    }

    fn reset(&mut self) {
        self.elapsed_time = 0;
        self.elapsed_time_since_last_message = 0;
        self.messages.clear();
        self.opened = false;
    }
}
