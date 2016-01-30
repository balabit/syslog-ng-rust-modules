use std::sync::Arc;

use Message;
use timer::TimerEvent;
use std::time::Duration;
use context::BaseContext;

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

    fn open(&mut self, context: &BaseContext) {
        trace!("Context: opening state; uuid={}", context.uuid());
        for i in context.actions() {
            i.on_opened(self, context);
        }
        self.opened = true;
    }

    fn close(&mut self, context: &BaseContext) {
        trace!("Context: closing state; uuid={}", context.uuid());
        for i in context.actions() {
            i.on_closed(self, context);
        }
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

    fn add_message(&mut self, message: Arc<Message>) {
        self.messages.push(message);
        self.elapsed_time_since_last_message = Duration::from_secs(0);
    }

    pub fn on_timer(&mut self, event: &TimerEvent, context: &BaseContext) {
        if self.is_open() {
            self.update_timers(event);
        }
        if context.conditions().is_closing(self) {
            self.close(context);
        }
    }

    pub fn on_message(&mut self, event: Arc<Message>, context: &BaseContext) {
        if self.is_open() {
            self.add_message(event);
        } else if context.conditions().is_opening(&event) {
            self.add_message(event);
            self.open(context);
        }

        if context.conditions().is_closing(self) {
            self.close(context);
        }
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
