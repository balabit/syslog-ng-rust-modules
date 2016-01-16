use std::rc::Rc;

use Message;
use MiliSec;
use timer::TimerEvent;
use context::BaseContext;

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

    pub fn close(&mut self, context: &BaseContext) {
        trace!("Context: closing state; uuid={}", context.uuid());
        for i in context.actions() {
            i.execute(self, context);
        }
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

    pub fn on_timer(&mut self, event: &TimerEvent, context: &BaseContext) {
        if self.is_open() {
            self.update_timers(event);
        }
        if context.conditions().is_closing(self) {
            self.close(context);
        }
    }

    pub fn on_message(&mut self, event: Rc<Message>, context: &BaseContext) {
        if self.is_open() {
            self.add_message(event);
        } else if context.conditions().is_opening(&event) {
            trace!("Context: opening state; uuid={}", context.uuid());
            self.add_message(event);
            self.open();
        }

        if context.conditions().is_closing(self) {
            self.close(context);
        }
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
