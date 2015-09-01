use config::action::ActionType;
use config::Context;
use serde::de::{
    Deserialize,
    Deserializer,
    MapVisitor,
    Error,
    Visitor
};
use uuid::Uuid;

const FIELDS: &'static [&'static str] = &["name", "uuid", "conditions", "actions"];

impl Deserialize for Context {
    fn deserialize<D>(deserializer: &mut D) -> Result<Context, D::Error>
        where D: Deserializer
    {
        deserializer.visit_struct("Context", FIELDS, ContextVisitor)
    }
}

enum Field {
    Name,
    Uuid,
    Conditions,
    Actions,
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
                    "name" => Ok(Field::Name),
                    "uuid" => Ok(Field::Uuid),
                    "conditions" => Ok(Field::Conditions),
                    "actions" => Ok(Field::Actions),
                    _ => Err(Error::syntax(&format!("Unexpected field: {}", value))),
                }
            }
        }

        deserializer.visit(FieldVisitor)
    }
}

struct ContextVisitor;

impl ContextVisitor {
    fn parse_uuid<V>(uuid: Option<String>) -> Result<Uuid, V::Error>
        where V: MapVisitor {
        match uuid {
            Some(value) => {
                match Uuid::parse_str(&value) {
                    Ok(uuid) => Ok(uuid),
                    Err(err) => {
                        return Err(Error::syntax(&format!("Failed to parse field 'uuid': uuid={} error={}", value, err)));
                    }
                }
            },
            None => {
                return Err(Error::missing_field("uuid"));
            }
        }
    }

    fn parse_actions<V>(actions: Option<Vec<ActionType>>) -> Result<Vec<ActionType>, V::Error>
        where V: MapVisitor {
        match actions {
            Some(actions) => Ok(actions),
            None => Ok(Vec::new())
        }
    }
}

impl Visitor for ContextVisitor {
    type Value = Context;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Context, V::Error>
        where V: MapVisitor
    {
        let mut name = None;
        let mut uuid: Option<String> = None;
        let mut conditions = None;
        let mut actions = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Field::Name) => { name = Some(try!(visitor.visit_value())); }
                Some(Field::Uuid) => { uuid = Some(try!(visitor.visit_value())); }
                Some(Field::Conditions) => { conditions = Some(try!(visitor.visit_value())); }
                Some(Field::Actions) => { actions = Some(try!(visitor.visit_value())); }
                None => break
            }
        }

        let uuid = try!(ContextVisitor::parse_uuid::<V>(uuid));
        let actions = try!(ContextVisitor::parse_actions::<V>(actions));

        try!(visitor.end());

        Ok(
            Context {
                name: name,
                uuid: uuid,
                conditions: conditions.unwrap(),
                actions: actions
            }
        )
    }
}

#[cfg(test)]
mod test {
    use action::{
        ActionType,
        MessageActionType
    };
    use conditions;
    use config::{
        Context,
        ContextBuilder
    };
    use serde_json::from_str;
    use uuid::Uuid;

    #[test]
    fn test_given_config_context_when_it_is_deserialized_then_we_get_the_right_results() {
        let text = r#"
        {
            "name": "TEST_NAME",
            "uuid": "86ca9f93-84fb-4813-b037-6526f7a585a3",
            "conditions": {
                "timeout": 100,
                "first_opens": true,
                "patterns": [
                    "PATTERN_NAME1",
                    "PATTERN_NAME2",
                    "f13dafee-cd14-4dda-995c-6ed476a21de3"
                ]
            },
            "actions": [
                {
                    "message": null
                }
            ]
        }
        "#;

        let result = from_str::<Context>(text);
        println!("{:?}", &result);
        let expected_name = "TEST_NAME".to_string();
        let expected_uuid = Uuid::parse_str("86ca9f93-84fb-4813-b037-6526f7a585a3").ok().unwrap();
        let expected_conditions = conditions::Builder::new(100).
                                                        first_opens(true).
                                                        patterns(vec![
                                                            "PATTERN_NAME1".to_string(),
                                                            "PATTERN_NAME2".to_string(),
                                                            "f13dafee-cd14-4dda-995c-6ed476a21de3".to_string()
                                                        ]).build();
        let expected_actions = vec![ActionType::Message(MessageActionType::new())];
        let expected_context = ContextBuilder::new(expected_uuid, expected_conditions)
                                                          .name(expected_name)
                                                          .actions(expected_actions)
                                                          .build();
        let context = result.ok().expect("Failed to deserialize a valid Context");
        assert_eq!(&expected_context, &context);
    }

    #[test]
    fn test_given_config_context_when_it_is_deserialized_and_only_the_required_fields_are_present_then_we_can_deserialize_it_successfully() {
        let text = r#"
        {
            "uuid": "86ca9f93-84fb-4813-b037-6526f7a585a3",
            "conditions": {
                "timeout": 100
            }
        }
        "#;

        let result = from_str::<Context>(text);
        println!("{:?}", &result);
        let expected_uuid = Uuid::parse_str("86ca9f93-84fb-4813-b037-6526f7a585a3").ok().unwrap();
        let expected_conditions = conditions::Builder::new(100).build();
        let expected_context = ContextBuilder::new(expected_uuid, expected_conditions).build();
        let context = result.ok().expect("Failed to deserialize a valid Context");
        assert_eq!(&expected_context, &context);
    }

    #[test]
    fn test_given_config_context_when_it_is_deserialized_and_the_uuid_is_invalid_then_we_report_an_error() {
        let text = r#"
        {
            "uuid": "this is an invalid uuid",
            "conditions": {
                "timeout": 100
            }
        }
        "#;

        let result = from_str::<Context>(text);
        println!("{:?}", &result);
        let _ = result.err().expect("Successfully deserialized an invalid Context (UUID is invalid)");
    }
}
