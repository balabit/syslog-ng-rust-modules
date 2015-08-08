use Message;
use state::State;

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
        self.is_max_size_reached(state) || self.is_closing_message(state) || self.is_any_timer_expired(state)
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

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use message;
    use state::State;
    use super::Builder;

    #[test]
    fn test_given_close_condition_when_an_unknown_message_received_then_it_is_ignored() {
        let timeout = 100;
        let msg_id = "1".to_string();
        let condition = Builder::new(timeout).patterns(vec![msg_id.clone()]).build();
        let msg_which_should_not_be_ignored = message::Builder::new(msg_id.clone()).build();
        let msg_which_should_be_ignored = message::Builder::new("2".to_string()).build();
        assert_true!(condition.ignore_message(&msg_which_should_be_ignored));
        assert_false!(condition.ignore_message(&msg_which_should_not_be_ignored));
    }

    #[test]
    fn test_given_condition_when_an_opening_message_is_received_then_the_state_becomes_opened() {
        let timeout = 100;
        let msg_id = "1".to_string();
        let condition = Builder::new(timeout).patterns(vec![msg_id.clone()]).first_opens(true).build();
        let msg_which_should_not_be_ignored = message::Builder::new(msg_id.clone()).build();
        let msg_which_should_be_ignored = message::Builder::new("2".to_string()).build();
        assert_false!(condition.is_opening(&msg_which_should_be_ignored));
        assert_true!(condition.is_opening(&msg_which_should_not_be_ignored));
    }

    #[test]
    fn test_given_condition_when_a_closing_message_is_received_then_the_state_becomes_closed() {
        let timeout = 100;
        let msg_id = "1".to_string();
        let mut state = State::new();
        let condition = Builder::new(timeout).patterns(vec!["1".to_string(), "2".to_string()]).last_closes(true).build();
        let msg_1 = message::Builder::new(msg_id.clone()).build();
        let msg_closing = Rc::new(message::Builder::new("2".to_string()).build());
        assert_true!(condition.is_opening(&msg_1));
        state.open();
        state.add_message(msg_closing);
        assert_true!(condition.is_closing(&mut state));
    }
}
