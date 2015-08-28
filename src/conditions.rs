use message::{Message, PatternId};
use MiliSec;
use state::State;

const FIRST_OPENS_DEFAULT: bool = false;
const LAST_CLOSES_DEFAULT: bool = false;

#[derive(Clone, Debug)]
pub struct Conditions {
    pub timeout: MiliSec,
    pub renew_timeout: Option<MiliSec>,
    pub first_opens: Option<bool>,
    pub last_closes: Option<bool>,
    pub max_size: Option<usize>,
    pub patterns: Vec<PatternId>
}

impl Conditions {
    fn new(timeout: MiliSec) -> Conditions {
        Conditions {
            timeout: timeout,
            renew_timeout: None,
            first_opens: None,
            last_closes: None,
            max_size: None,
            patterns: Vec::new()
        }
    }

    pub fn is_opening(&self, message: &Message) -> bool {
        if self.first_opens.unwrap_or(FIRST_OPENS_DEFAULT) {
            self.patterns.first().unwrap() == message.uuid()
        } else {
            true
        }
    }

    pub fn is_closing(&self, state: &State) -> bool {
        self.is_max_size_reached(state) || self.is_closing_message(state) || self.is_any_timer_expired(state)
    }

    fn is_max_size_reached(&self, state: &State) -> bool {
        self.max_size.map_or(false, |max_size| state.messages().len() >= max_size)
    }

    fn is_closing_message(&self, state: &State) -> bool {
        if self.last_closes.unwrap_or(LAST_CLOSES_DEFAULT) {
            self.patterns.last().unwrap() == state.messages().last().unwrap().uuid()
        } else {
            false
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
    pub fn new(timeout: MiliSec) -> Builder {
        Builder{
            conditions: Conditions::new(timeout)
        }
    }

    pub fn renew_timeout(&mut self, timeout: MiliSec) -> &mut Builder {
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

    pub fn patterns(&mut self, patterns: Vec<PatternId>) -> &mut Builder {
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
    use message::{PatternId};
    use state::State;
    use super::Builder;

    #[test]
    fn test_given_condition_when_an_opening_message_is_received_then_the_state_becomes_opened() {
        let timeout = 100;
        let msg_id1 = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let msg_id2 = "21eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let patterns = vec![
            PatternId::Uuid(msg_id1.clone()),
        ];
        let condition = Builder::new(timeout).patterns(patterns).first_opens(true).build();
        let msg_which_should_not_be_ignored = message::Builder::new(&msg_id1).build();
        let msg_which_should_be_ignored = message::Builder::new(&msg_id2).build();
        assert_false!(condition.is_opening(&msg_which_should_be_ignored));
        assert_true!(condition.is_opening(&msg_which_should_not_be_ignored));
    }

    #[test]
    fn test_given_condition_when_a_closing_message_is_received_then_the_state_becomes_closed() {
        let timeout = 100;
        let msg_id1 = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let msg_id2 = "21eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let patterns = vec![
            PatternId::Uuid(msg_id1.clone()),
            PatternId::Uuid(msg_id2.clone()),
        ];
        let mut state = State::new();
        let condition = Builder::new(timeout).patterns(patterns).last_closes(true).build();
        let msg_1 = message::Builder::new(&msg_id1).build();
        let msg_closing = Rc::new(message::Builder::new(&msg_id2).build());
        assert_true!(condition.is_opening(&msg_1));
        state.open();
        state.add_message(msg_closing);
        assert_true!(condition.is_closing(&mut state));
    }
}

mod deser {

    use super::Conditions;
    use serde;
    use serde::de::Deserialize;

    impl serde::Deserialize for Conditions {
        fn deserialize<D>(deserializer: &mut D) -> Result<Conditions, D::Error>
            where D: serde::de::Deserializer
        {
            deserializer.visit_struct("Conditions", &[], ConditionsVisitor)
        }
    }

    enum Field {
        TIMEOUT,
        RENEW_TIMEOUT,
        FIRST_OPENS,
        LAST_CLOSES,
        MAX_SIZE,
        PATTERNS,
    }

    impl serde::Deserialize for Field {
        fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
            where D: serde::de::Deserializer
        {
            struct FieldVisitor;

            impl serde::de::Visitor for FieldVisitor {
                type Value = Field;

                fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                    where E: serde::de::Error
                {
                    match value {
                        "timeout" => Ok(Field::TIMEOUT),
                        "renew_timeout" => Ok(Field::RENEW_TIMEOUT),
                        "first_opens" => Ok(Field::FIRST_OPENS),
                        "last_closes" => Ok(Field::LAST_CLOSES),
                        "max_size" => Ok(Field::MAX_SIZE),
                        "patterns" => Ok(Field::PATTERNS),
                        name @ _ => Err(serde::de::Error::syntax(&format!("Unexpected field: {}", name))),
                    }
                }
            }

            deserializer.visit(FieldVisitor)
        }
    }

    struct ConditionsVisitor;

    impl serde::de::Visitor for ConditionsVisitor {
        type Value = Conditions;

        fn visit_map<V>(&mut self, mut visitor: V) -> Result<Conditions, V::Error>
            where V: serde::de::MapVisitor
        {
            let mut timeout = None;
            let mut renew_timeout = None;
            let mut first_opens = None;
            let mut last_closes = None;
            let mut max_size = None;
            let mut patterns = None;
            let mut values = None;

            loop {
                match try!(visitor.visit_key()) {
                    Some(Field::TIMEOUT) => { timeout = Some(try!(visitor.visit_value())); }
                    Some(Field::RENEW_TIMEOUT) => { renew_timeout = Some(try!(visitor.visit_value())); }
                    Some(Field::FIRST_OPENS) => { first_opens = Some(try!(visitor.visit_value())); }
                    Some(Field::LAST_CLOSES) => { last_closes = Some(try!(visitor.visit_value())); }
                    Some(Field::MAX_SIZE) => { max_size = Some(try!(visitor.visit_value())); }
                    Some(Field::PATTERNS) => { patterns = Some(try!(visitor.visit_value())); }
                    None => { break; }
                }
            }

            let timeout = try!(timeout.ok_or(visitor.missing_field("timeout")));

            try!(visitor.end());

            Ok(
                Conditions {
                    timeout: timeout,
                    renew_timeout: renew_timeout,
                    first_opens: first_opens,
                    last_closes: last_closes,
                    max_size: max_size,
                    patterns: patterns.unwrap_or(Vec::new())
                }
            )
        }
    }
}
