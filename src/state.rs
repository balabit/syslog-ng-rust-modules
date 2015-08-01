use std::rc::Rc;

use Message;
use MiliSec;
use TimerEvent;

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
        self.opened = false;
    }

    pub fn elapsed_time(&self) -> MiliSec {
        self.elapsed_time
    }

    pub fn elapsed_time_since_last_message(&self) -> MiliSec {
        self.elapsed_time_since_last_message
    }

    pub fn messages(&self) -> &Vec<Rc<Message>> {
        &self.messages
    }

    pub fn messages_mut(&mut self) -> &mut Vec<Rc<Message>> {
        &mut self.messages
    }

    pub fn add_message(&mut self, message: Rc<Message>) {
        self.messages.push(message);
    }

    pub fn on_timer(&mut self, event: &TimerEvent) {
        let delta = event.0;
        self.elapsed_time += delta;
        self.elapsed_time_since_last_message += delta;
    }
}
