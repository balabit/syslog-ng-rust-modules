use std::rc::Rc;

use action::ExecResult;
use context::BaseContext;
use Message;
use state::State;
use TimerEvent;

#[derive(Clone, Debug)]
pub struct Conditions {
    pub timeout: u32,
    pub renew_timeout: Option<u32>,
    pub first_opens: Option<bool>,
    pub last_closes: Option<bool>,
    pub max_size: Option<usize>,
    pub patterns: Vec<String>
}

impl Conditions {
    fn new(timeout: u32) -> Conditions {
        Conditions {
            timeout: timeout,
            renew_timeout: None,
            first_opens: None,
            last_closes: None,
            max_size: None,
            patterns: Vec::new()
        }
    }

    pub fn ignore_message(&self, message: &Message) -> bool {
        !self.patterns.contains(message.uuid()) && self.patterns.len() > 0
    }

    pub fn is_opening(&self, message: &Message) -> bool {
        self.first_opens.map_or(true, |first_message_opens_the_context| {
            if first_message_opens_the_context {
                self.patterns.first().unwrap() == message.uuid()
            } else {
                true
            }
        })
    }

    pub fn is_closing(&self, state: &State) -> bool {
        println!("checking close");
        self.is_max_size_reached(state) || self.is_closing_message(state)
    }

    fn is_max_size_reached(&self, state: &State) -> bool {
        self.max_size.map_or(false, |max_size| state.messages().len() >= max_size)
    }

    fn is_closing_message(&self, state: &State) -> bool {
        self.last_closes.map_or(false, |last_message_closes_the_context| {
            if last_message_closes_the_context {
                self.patterns.last().unwrap() == state.messages().last().unwrap().uuid()
            } else {
                false
            }
        })
    }

    pub fn on_timer(&self, event: &TimerEvent, state: &mut State, context: &BaseContext) -> Option<Vec<ExecResult>> {
        state.on_timer(event);
        if self.is_any_timer_expired(state) {
            println!("closing state");
            state.close(context)
        } else {
            None
        }
    }

    fn is_any_timer_expired(&self, state: &State) -> bool {
        self.is_timeout_expired(state) || self.is_renew_timeout_expired(state)
    }

    fn is_timeout_expired(&self, state: &State) -> bool {
        state.elapsed_time() >= self.timeout
    }

    fn is_renew_timeout_expired(&self, state: &State) -> bool {
        self.renew_timeout.map_or(false, |renew_timeout| {
            state.elapsed_time_since_last_message() >= renew_timeout
        })
    }
}

pub struct Builder {
    conditions: Conditions
}

impl Builder {
    pub fn new(timeout: u32) -> Builder {
        Builder{
            conditions: Conditions::new(timeout)
        }
    }

    pub fn renew_timeout(&mut self, timeout: u32) -> &mut Builder {
        self.conditions.renew_timeout = Some(timeout);
        self
    }

    pub fn first_opens(&mut self, first_opens: bool) -> &mut Builder {
        self.conditions.first_opens = Some(first_opens);
        self
    }

    pub fn last_closes(&mut self, last_closes: bool) -> &mut Builder {
        self.conditions.last_closes = Some(last_closes);
        self
    }
    pub fn max_size(&mut self, max_size: usize) -> &mut Builder {
        self.conditions.max_size = Some(max_size);
        self
    }

    pub fn patterns(&mut self, patterns: Vec<String>) -> &mut Builder {
        self.conditions.patterns = patterns;
        self
    }

    pub fn build(&mut self) -> Conditions {
        self.conditions.clone()
    }
}
