// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::time::Duration;

const FIRST_OPENS_DEFAULT: bool = false;
const LAST_CLOSES_DEFAULT: bool = false;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conditions {
    pub timeout: Duration,
    pub renew_timeout: Option<Duration>,
    pub first_opens: bool,
    pub last_closes: bool,
    pub max_size: Option<usize>,
}

impl Conditions {
    fn new(timeout: Duration) -> Conditions {
        Conditions {
            timeout: timeout,
            renew_timeout: None,
            first_opens: FIRST_OPENS_DEFAULT,
            last_closes: LAST_CLOSES_DEFAULT,
            max_size: None,
        }
    }
}

pub struct ConditionsBuilder {
    conditions: Conditions,
}

impl ConditionsBuilder {
    pub fn new(timeout: Duration) -> ConditionsBuilder {
        ConditionsBuilder { conditions: Conditions::new(timeout) }
    }

    pub fn renew_timeout(&mut self, timeout: Duration) -> &mut ConditionsBuilder {
        self.conditions.renew_timeout = Some(timeout);
        self
    }

    pub fn first_opens(&mut self, first_opens: bool) -> &mut ConditionsBuilder {
        self.conditions.first_opens = first_opens;
        self
    }

    pub fn last_closes(&mut self, last_closes: bool) -> &mut ConditionsBuilder {
        self.conditions.last_closes = last_closes;
        self
    }
    pub fn max_size(&mut self, max_size: usize) -> &mut ConditionsBuilder {
        self.conditions.max_size = Some(max_size);
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
    use std::sync::Arc;

    use message::MessageBuilder;
    use state::State;
    use super::ConditionsBuilder;
    use context::BaseContextBuilder;
    use uuid::Uuid;
    use std::time::Duration;
    use test_utils::{MockResponseSender, MockTemplate};
    use Message;

    #[test]
    fn test_given_condition_when_an_opening_message_is_received_then_the_state_becomes_opened() {
        let timeout = Duration::from_millis(100);
        let msg_id1 = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_owned();
        let msg_id2 = "21eaf6f8-0640-460f-aee2-a72d2f2ab258".to_owned();
        let patterns = vec![
            msg_id1.clone(),
        ];
        let condition = ConditionsBuilder::new(timeout)
                            .first_opens(true)
                            .build();
        let msg_which_should_not_be_ignored = MessageBuilder::new(&msg_id1, "message").build();
        let msg_which_should_be_ignored = MessageBuilder::new(&msg_id2, "message").build();
        let base = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), condition).patterns(patterns).build();
        assert_false!(base.is_opening(&msg_which_should_be_ignored));
        assert_true!(base.is_opening(&msg_which_should_not_be_ignored));
    }

    #[test]
    fn test_given_condition_when_a_closing_message_is_received_then_the_state_becomes_closed() {
        let mut responder = MockResponseSender::default();
        let timeout = Duration::from_millis(100);
        let msg_id1 = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_owned();
        let msg_id2 = "21eaf6f8-0640-460f-aee2-a72d2f2ab258".to_owned();
        let patterns = vec![
            msg_id1.clone(),
            msg_id2.clone(),
        ];
        let mut state = State::new();
        let conditions = ConditionsBuilder::new(timeout)
                             .last_closes(true)
                             .build();
        let context = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), conditions).patterns(patterns).build();
        let msg_opening = Arc::new(MessageBuilder::new(&msg_id1, "message").build());
        let msg_closing = Arc::new(MessageBuilder::new(&msg_id2, "message").build());
        assert_false!(state.is_open());
        context.on_message(msg_opening, &mut state, &mut responder);
        assert_true!(state.is_open());
        context.on_message(msg_closing, &mut state, &mut responder);
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
        let conditions: Conditions = conditions.expect("Failed to deserialize a Conditions \
                                                        struct with only a timeout field");
        assert_eq!(conditions.timeout, Duration::from_millis(100));
    }

    #[test]
    fn test_given_conditions_in_json_when_we_have_all_fields_then_we_get_the_expected_result() {
        let json = r#"
        {
            "timeout": 100,
            "renew_timeout": 50,
            "first_opens": true,
            "last_closes": false,
            "max_size": 42
        }
        "#;

        let conditions = from_str(json);
        println!("{:?}", &conditions);
        let conditions: Conditions = conditions.expect("Failed to deserialize a Conditions struct");
        assert_eq!(conditions.timeout, Duration::from_millis(100));
        assert_eq!(conditions.renew_timeout, Some(Duration::from_millis(50)));
        assert_eq!(conditions.first_opens, true);
        assert_eq!(conditions.last_closes, false);
        assert_eq!(conditions.max_size, Some(42));
    }

    #[test]
    fn test_given_condition_when_there_are_no_patterns_then_any_message_can_open_the_context() {
        let timeout = Duration::from_millis(100);
        let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_owned();
        let condition = ConditionsBuilder::new(timeout).build();
        let base = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), condition).build();
        let msg = MessageBuilder::new(&msg_id, "message").build();
        assert_true!(base.is_opening(&msg));
    }

    #[test]
    fn test_given_condition_when_first_opens_is_set_then_the_right_message_can_open_the_context
        () {
        let timeout = Duration::from_millis(100);
        let patterns = vec![
                "p1".to_owned(),
                "p2".to_owned(),
                "p3".to_owned(),
        ];
        let uuid = "e4f3f8b2-3135-4916-a5ea-621a754dab0d".to_owned();
        let msg_id = "p1".to_owned();
        let condition = ConditionsBuilder::new(timeout)
                            .first_opens(true)
                            .build();
        let base = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), condition).patterns(patterns).build();
        let msg = MessageBuilder::new(&uuid, "message").name(Some(msg_id)).build();
        assert_true!(base.is_opening(&msg));
    }

    #[test]
    fn test_given_conditions_when_last_closes_is_set_and_the_message_has_a_name_then_we_check_that_name
        () {
        let mut responder = MockResponseSender::default();
        let timeout = Duration::from_millis(100);
        let patterns = vec!["p1".to_owned(), "p2".to_owned()];
        let p1_uuid = "e4f3f8b2-3135-4916-a5ea-621a754dab0d".to_owned();
        let p2_uuid = "f4f3f8b2-3135-4916-a5ea-621a754dab0d".to_owned();
        let p1 = "p1".to_owned();
        let p2 = "p2".to_owned();
        let mut state = State::new();
        let conditions = ConditionsBuilder::new(timeout)
                             .first_opens(true)
                             .last_closes(true)
                             .build();
        let p1_msg = MessageBuilder::new(&p1_uuid, "message").name(Some(p1)).build();
        let p2_msg = MessageBuilder::new(&p2_uuid, "message").name(Some(p2)).build();
        let context = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), conditions).patterns(patterns).build();
        assert_false!(state.is_open());
        context.on_message(Arc::new(p1_msg), &mut state, &mut responder);
        context.on_message(Arc::new(p2_msg), &mut state, &mut responder);
        assert_false!(state.is_open());
    }

    #[test]
    fn test_given_condition_when_first_opens_is_set_but_there_are_no_patterns_then_we_do_not_panic
        () {
        let mut responder = MockResponseSender::default();
        let msg = MessageBuilder::new("e4f3f8b2-3135-4916-a5ea-621a754dab0d", "message")
                      .name(Some("p1"))
                      .build();
        let conditions = ConditionsBuilder::new(Duration::from_millis(100))
                             .first_opens(true)
                             .build();
        let context = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), conditions).patterns(Vec::new()).build();
        let mut state = State::new();
        context.on_message(Arc::new(msg), &mut state, &mut responder);
    }

    #[test]
    fn test_given_condition_when_last_closes_is_set_but_there_are_no_patterns_then_we_do_not_panic
        () {
        let mut responder = MockResponseSender::default();
        let msg = MessageBuilder::new("e4f3f8b2-3135-4916-a5ea-621a754dab0d", "message")
                      .name(Some("p1"))
                      .build();
        let conditions = ConditionsBuilder::new(Duration::from_millis(100))
                             .last_closes(true)
                             .build();
        let context = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), conditions).build();
        let mut state = State::new();
        context.on_message(Arc::new(msg), &mut state, &mut responder);
    }
}

mod deser {
    use super::{Conditions, FIRST_OPENS_DEFAULT, LAST_CLOSES_DEFAULT};
    use serde::de::{Deserialize, Deserializer, Error, MapVisitor, Visitor};
    use std::time::Duration;
    use duration::SerializableDuration;

    impl Deserialize for Conditions {
        fn deserialize<D>(deserializer: &mut D) -> Result<Conditions, D::Error>
            where D: Deserializer
        {
            deserializer.deserialize_struct("Conditions", &[], ConditionsVisitor)
        }
    }

    enum Field {
        Timeout,
        RenewTimeout,
        FirstOpens,
        LastCloses,
        MaxSize,
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
                        _ => Err(E::custom(format!("Unexpected field: {}", value))),
                    }
                }
            }

            deserializer.deserialize(FieldVisitor)
        }
    }

    struct ConditionsVisitor;

    impl Visitor for ConditionsVisitor {
        type Value = Conditions;

        fn visit_map<V>(&mut self, mut visitor: V) -> Result<Conditions, V::Error>
            where V: MapVisitor
        {
            let mut timeout: Option<SerializableDuration> = None;
            let mut renew_timeout: Option<SerializableDuration> = None;
            let mut first_opens = FIRST_OPENS_DEFAULT;
            let mut last_closes = LAST_CLOSES_DEFAULT;
            let mut max_size = None;

            while let Some(field) = try!(visitor.visit_key()) {
                match field {
                    Field::Timeout => timeout = Some(try!(visitor.visit_value())),
                    Field::RenewTimeout => renew_timeout = Some(try!(visitor.visit_value())),
                    Field::FirstOpens => first_opens = try!(visitor.visit_value()),
                    Field::LastCloses => last_closes = try!(visitor.visit_value()),
                    Field::MaxSize => max_size = Some(try!(visitor.visit_value())),
                }
            }

            let timeout: Duration = match timeout {
                Some(timeout) => timeout.0,
                None => return visitor.missing_field("timeout"),
            };

            let renew_timeout = renew_timeout.map(|timeout| timeout.0);

            try!(visitor.end());

            Ok(Conditions {
                timeout: timeout,
                renew_timeout: renew_timeout,
                first_opens: first_opens,
                last_closes: last_closes,
                max_size: max_size,
            })
        }
    }
}
