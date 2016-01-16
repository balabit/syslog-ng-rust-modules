use message::Message;
use MiliSec;
use state::State;

const FIRST_OPENS_DEFAULT: bool = false;
const LAST_CLOSES_DEFAULT: bool = false;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conditions {
    pub timeout: MiliSec,
    pub renew_timeout: Option<MiliSec>,
    pub first_opens: Option<bool>,
    pub last_closes: Option<bool>,
    pub max_size: Option<usize>,
    pub patterns: Vec<String>,
}

impl Conditions {
    fn new(timeout: MiliSec) -> Conditions {
        Conditions {
            timeout: timeout,
            renew_timeout: None,
            first_opens: None,
            last_closes: None,
            max_size: None,
            patterns: Vec::new(),
        }
    }

    pub fn is_opening(&self, message: &Message) -> bool {
        if self.first_opens.unwrap_or(FIRST_OPENS_DEFAULT) {
            message.ids().any(|x| x == self.patterns.first().unwrap())
        } else {
            true
        }
    }

    pub fn is_closing(&self, state: &State) -> bool {
        trace!("Conditions: shoud we close this context?");
        state.is_open() && self.is_closing_condition_met(state)
    }

    fn is_closing_condition_met(&self, state: &State) -> bool {
        self.is_max_size_reached(state) ||
        self.is_closing_message(state) ||
        self.is_any_timer_expired(state)
    }

    fn is_max_size_reached(&self, state: &State) -> bool {
        self.max_size.map_or(false, |max_size| state.messages().len() >= max_size)
    }

    fn is_closing_message(&self, state: &State) -> bool {
        if self.last_closes.unwrap_or(LAST_CLOSES_DEFAULT) {
            let last_message = state.messages().last().unwrap();
            last_message.ids().any(|x| x == self.patterns.last().unwrap())
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
        self.renew_timeout.map_or(false,
                                  |renew_timeout| {
                                      state.elapsed_time_since_last_message() >= renew_timeout
                                  })
    }
}

pub struct ConditionsBuilder {
    conditions: Conditions,
}

impl ConditionsBuilder {
    pub fn new(timeout: MiliSec) -> ConditionsBuilder {
        ConditionsBuilder { conditions: Conditions::new(timeout) }
    }

    pub fn renew_timeout(&mut self, timeout: MiliSec) -> &mut ConditionsBuilder {
        self.conditions.renew_timeout = Some(timeout);
        self
    }

    pub fn first_opens(&mut self, first_opens: bool) -> &mut ConditionsBuilder {
        self.conditions.first_opens = Some(first_opens);
        self
    }

    pub fn last_closes(&mut self, last_closes: bool) -> &mut ConditionsBuilder {
        self.conditions.last_closes = Some(last_closes);
        self
    }
    pub fn max_size(&mut self, max_size: usize) -> &mut ConditionsBuilder {
        self.conditions.max_size = Some(max_size);
        self
    }

    pub fn patterns(&mut self, patterns: Vec<String>) -> &mut ConditionsBuilder {
        self.conditions.patterns = patterns;
        self
    }

    pub fn build(&mut self) -> Conditions {
        self.conditions.clone()
    }
}

#[cfg(test)]
mod test {
    use serde_json::from_str;
    use super::Conditions;
    use std::rc::Rc;

    use message::MessageBuilder;
    use state::State;
    use super::ConditionsBuilder;
    use context::BaseContextBuilder;
    use uuid::Uuid;

    #[test]
    fn test_given_condition_when_an_opening_message_is_received_then_the_state_becomes_opened() {
        let timeout = 100;
        let msg_id1 = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let msg_id2 = "21eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let patterns = vec![
            msg_id1.clone(),
        ];
        let condition = ConditionsBuilder::new(timeout)
                            .patterns(patterns)
                            .first_opens(true)
                            .build();
        let msg_which_should_not_be_ignored = MessageBuilder::new(&msg_id1, "message").build();
        let msg_which_should_be_ignored = MessageBuilder::new(&msg_id2, "message").build();
        assert_false!(condition.is_opening(&msg_which_should_be_ignored));
        assert_true!(condition.is_opening(&msg_which_should_not_be_ignored));
    }

    #[test]
    fn test_given_condition_when_a_closing_message_is_received_then_the_state_becomes_closed() {
        let timeout = 100;
        let msg_id1 = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let msg_id2 = "21eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let patterns = vec![
            msg_id1.clone(),
            msg_id2.clone(),
        ];
        let mut state = State::new();
        let conditions = ConditionsBuilder::new(timeout)
                            .patterns(patterns)
                            .last_closes(true)
                            .build();
        let context = BaseContextBuilder::new(Uuid::new_v4(), conditions).build();
        let msg_opening = Rc::new(MessageBuilder::new(&msg_id1, "message").build());
        let msg_closing = Rc::new(MessageBuilder::new(&msg_id2, "message").build());
        assert_false!(state.is_open());
        state.on_message(msg_opening, &context);
        assert_true!(state.is_open());
        state.on_message(msg_closing, &context);
        assert_false!(state.is_open());
    }

    #[test]
    fn test_given_conditions_in_json_when_we_have_only_the_required_ones_then_we_get_the_expected_result
        () {
        let json = r#"
        {
            "timeout": 100
        }
        "#;

        let conditions = from_str(json);
        println!("{:?}", &conditions);
        let conditions: Conditions = conditions.ok().expect("Failed to deserialize a Conditions \
                                                             struct with only a timeout field");
        assert_eq!(conditions.timeout, 100);
    }

    #[test]
    fn test_given_conditions_in_json_when_we_have_all_fields_then_we_get_the_expected_result() {
        let json = r#"
        {
            "timeout": 100,
            "renew_timeout": 50,
            "first_opens": true,
            "last_closes": false,
            "max_size": 42,
            "patterns": [
                "1f78c9f1-cd33-4f83-bbcd-9d59f73094d5",
                "2f78c9f1-cd33-4f83-bbcd-9d59f73094d5",
                "PATTERN_NAME"
            ]
        }
        "#;

        let expected_patterns = vec![
                "1f78c9f1-cd33-4f83-bbcd-9d59f73094d5".to_string(),
                "2f78c9f1-cd33-4f83-bbcd-9d59f73094d5".to_string(),
                "PATTERN_NAME".to_string(),
        ];
        let conditions = from_str(json);
        println!("{:?}", &conditions);
        let conditions: Conditions = conditions.ok()
                                               .expect("Failed to deserialize a Conditions struct");
        assert_eq!(conditions.timeout, 100);
        assert_eq!(conditions.renew_timeout, Some(50));
        assert_eq!(conditions.first_opens, Some(true));
        assert_eq!(conditions.last_closes, Some(false));
        assert_eq!(conditions.max_size, Some(42));
        assert_eq!(conditions.patterns, expected_patterns);
    }

    #[test]
    fn test_given_condition_when_there_are_no_patterns_then_any_message_can_open_the_context() {
        let timeout = 100;
        let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let condition = ConditionsBuilder::new(timeout).build();
        let msg = MessageBuilder::new(&msg_id, "message").build();
        assert_true!(condition.is_opening(&msg));
    }

    #[test]
    fn test_given_condition_when_first_opens_is_set_then_the_right_message_can_open_the_context
                                                                                                () {
        let timeout = 100;
        let patterns = vec![
                "p1".to_string(),
                "p2".to_string(),
                "p3".to_string(),
        ];
        let uuid = "e4f3f8b2-3135-4916-a5ea-621a754dab0d".to_string();
        let msg_id = "p1".to_string();
        let condition = ConditionsBuilder::new(timeout)
                            .patterns(patterns)
                            .first_opens(true)
                            .build();
        let msg = MessageBuilder::new(&uuid, "message").name(Some(&msg_id)).build();
        assert_true!(condition.is_opening(&msg));
    }

    #[test]
    fn test_given_conditions_when_last_closes_is_set_and_the_message_has_a_name_then_we_check_that_name
        () {
        let timeout = 100;
        let patterns = vec!["p1".to_string(), "p2".to_string()];
        let p1_uuid = "e4f3f8b2-3135-4916-a5ea-621a754dab0d".to_string();
        let p2_uuid = "f4f3f8b2-3135-4916-a5ea-621a754dab0d".to_string();
        let p1 = "p1".to_string();
        let p2 = "p2".to_string();
        let mut state = State::new();
        let conditions = ConditionsBuilder::new(timeout)
                            .patterns(patterns)
                            .first_opens(true)
                            .last_closes(true)
                            .build();
        let p1_msg = MessageBuilder::new(&p1_uuid, "message").name(Some(&p1)).build();
        let p2_msg = MessageBuilder::new(&p2_uuid, "message").name(Some(&p2)).build();
        let context = BaseContextBuilder::new(Uuid::new_v4(), conditions).build();
        assert_false!(state.is_open());
        state.on_message(Rc::new(p1_msg), &context);
        state.on_message(Rc::new(p2_msg), &context);
        assert_false!(state.is_open());
    }
}

mod deser {
    use MiliSec;
    use super::Conditions;
    use serde::de::{Deserialize, Deserializer, Error, MapVisitor, Visitor};

    impl Deserialize for Conditions {
        fn deserialize<D>(deserializer: &mut D) -> Result<Conditions, D::Error>
            where D: Deserializer
        {
            deserializer.visit_struct("Conditions", &[], ConditionsVisitor)
        }
    }

    enum Field {
        Timeout,
        RenewTimeout,
        FirstOpens,
        LastCloses,
        MaxSize,
        Patterns,
    }

    impl Deserialize for Field {
        fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
            where D: Deserializer
        {
            struct FieldVisitor;

            impl Visitor for FieldVisitor {
                type Value = Field;

                fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                    where E: Error
                {
                    match value {
                        "timeout" => Ok(Field::Timeout),
                        "renew_timeout" => Ok(Field::RenewTimeout),
                        "first_opens" => Ok(Field::FirstOpens),
                        "last_closes" => Ok(Field::LastCloses),
                        "max_size" => Ok(Field::MaxSize),
                        "patterns" => Ok(Field::Patterns),
                        name @ _ => Err(Error::syntax(&format!("Unexpected field: {}", name))),
                    }
                }
            }

            deserializer.visit(FieldVisitor)
        }
    }

    struct ConditionsVisitor;

    impl Visitor for ConditionsVisitor {
        type Value = Conditions;

        fn visit_map<V>(&mut self, mut visitor: V) -> Result<Conditions, V::Error>
            where V: MapVisitor
        {
            let mut timeout: Option<MiliSec> = None;
            let mut renew_timeout = None;
            let mut first_opens = None;
            let mut last_closes = None;
            let mut max_size = None;
            let mut patterns = None;

            loop {
                match try!(visitor.visit_key()) {
                    Some(Field::Timeout) => {
                        timeout = Some(try!(visitor.visit_value()));
                    }
                    Some(Field::RenewTimeout) => {
                        renew_timeout = Some(try!(visitor.visit_value()));
                    }
                    Some(Field::FirstOpens) => {
                        first_opens = Some(try!(visitor.visit_value()));
                    }
                    Some(Field::LastCloses) => {
                        last_closes = Some(try!(visitor.visit_value()));
                    }
                    Some(Field::MaxSize) => {
                        max_size = Some(try!(visitor.visit_value()));
                    }
                    Some(Field::Patterns) => {
                        patterns = Some(try!(visitor.visit_value()));
                    }
                    None => {
                        break;
                    }
                }
            }

            let timeout: MiliSec = match timeout {
                Some(timeout) => timeout,
                None => return visitor.missing_field("timeout"),
            };

            try!(visitor.end());

            Ok(Conditions {
                timeout: timeout,
                renew_timeout: renew_timeout,
                first_opens: first_opens,
                last_closes: last_closes,
                max_size: max_size,
                patterns: patterns.unwrap_or(Vec::new()),
            })
        }
    }
}
