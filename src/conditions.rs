use std::rc::Rc;

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

    pub fn on_message(&mut self, message: Rc<Message>, state: &mut State) {
        if !self.patterns.contains(message.get("uuid").unwrap()) && self.patterns.len() > 0 {
            println!("ignoring");
            return;
        }

        if state.is_open() {
            state.add_message(message);
            if self.is_closing(state) {
                state.close()
            }
        } else if self.is_opening(&message) {
            state.add_message(message);
            state.open();
        }
    }

    fn is_max_size_reached(&self, state: &State) -> bool {
        self.max_size.map_or(false, |max_size| state.messages().len() >= max_size)
    }

    fn is_closing_message(&self, state: &State) -> bool {
        self.last_closes.map_or(false, |closes| {
            if closes {
                self.patterns.last().map_or(false, |pattern| {
                    pattern == state.messages().last().unwrap().get("uuid").unwrap()
                })
            } else {
                false
            }
        })
    }

    fn is_opening(&self, message: &Message) -> bool {
        let found = self.patterns.contains(message.get("uuid").unwrap());
        println!("found: {}", found);
        self.first_opens.map_or(found, |first| {
            if first {
                self.patterns.first().map_or(false, |pattern| {
                    pattern == message.get("uuid").unwrap()
                })
            } else {
                found
            }
        })
    }

    fn is_closing(&self, state: &State) -> bool {
        println!("checking close");
        self.is_max_size_reached(state) || self.is_closing_message(state)
    }

    pub fn on_timer(&mut self, event: &TimerEvent, state: &mut State) {
        state.on_timer(event);
        if self.is_any_timer_expired(state) {
            println!("closing state");
            state.close()
        }
    }

    fn is_timeout_expired(&self, state: &State) -> bool {
        state.elapsed_time() >= self.timeout
    }

    fn is_renew_timeout_expired(&self, state: &State) -> bool {
        self.renew_timeout.map_or(false, |renew_timeout| {
            state.elapsed_time_since_last_message() >= renew_timeout
        })
    }

    fn is_any_timer_expired(&self, state: &State) -> bool {
        self.is_timeout_expired(state) || self.is_renew_timeout_expired(state)
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
