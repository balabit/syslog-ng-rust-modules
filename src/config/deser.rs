use config::Context;
use serde::de::{Deserialize, Deserializer, MapVisitor, Error, Visitor};

use handlebars::Template;
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
    ContextId,
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
                    "context_id" => Ok(Field::ContextId),
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
        where V: MapVisitor
    {
        match uuid {
            Some(value) => {
                match Uuid::parse_str(&value) {
                    Ok(uuid) => Ok(uuid),
                    Err(err) => {
                        return Err(Error::syntax(&format!("Failed to parse field 'uuid': \
                                                           uuid={} error={}",
                                                          value,
                                                          err)));
                    }
                }
            }
            None => {
                return Err(Error::missing_field("uuid"));
            }
        }
    }

    fn deser_context_id<V>(context_id: Option<String>,
                           uuid: &Uuid)
                           -> Result<Option<Template>, V::Error>
        where V: MapVisitor
    {
        if let Some(context_id) = context_id {
            ContextVisitor::parse_context_id::<V>(context_id, uuid)
        } else {
            Ok(None)
        }
    }

    fn parse_context_id<V>(context_id: String, uuid: &Uuid) -> Result<Option<Template>, V::Error>
        where V: MapVisitor
    {
        match Template::compile(context_id) {
            Ok(context_id) => Ok(Some(context_id)),
            Err(err) => {
                let errmsg = format!("Invalid handlebars template in 'context_id' field: uuid={} \
                                      error={}",
                                     uuid,
                                     err);
                return Err(Error::syntax(&errmsg));
            }
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
        let mut context_id: Option<String> = None;
        let mut actions = None;

        while let Some(field) = try!(visitor.visit_key()) {
            match field {
                Field::Name => name = Some(try!(visitor.visit_value())),
                Field::Uuid => uuid = Some(try!(visitor.visit_value())),
                Field::Conditions => conditions = Some(try!(visitor.visit_value())),
                Field::ContextId => context_id = Some(try!(visitor.visit_value())),
                Field::Actions => actions = Some(try!(visitor.visit_value())),
            }
        }

        let uuid = try!(ContextVisitor::parse_uuid::<V>(uuid));
        let context_id = try!(ContextVisitor::deser_context_id::<V>(context_id, &uuid));
        let actions = actions.unwrap_or(Vec::new());

        try!(visitor.end());

        Ok(Context {
            name: name,
            uuid: uuid,
            conditions: conditions.unwrap(),
            context_id: context_id,
            actions: actions,
        })
    }
}

#[cfg(test)]
mod test {
    use config::action::{ActionType, ExecCondition};
    use config::action::message::MessageActionBuilder;
    use conditions::ConditionsBuilder;
    use config::Context;
    use handlebars::Template;
    use serde_json::from_str;
    use uuid::Uuid;
    use std::time::Duration;

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
                    "message": {
                        "uuid": "uuid1",
                        "when": {
                            "on_closed": true,
                            "on_opened": false
                        },
                        "message": "message"
                    }
                }
            ]
        }
        "#;

        let result = from_str::<Context>(text);
        println!("{:?}", &result);
        let expected_name = "TEST_NAME".to_string();
        let expected_uuid = Uuid::parse_str("86ca9f93-84fb-4813-b037-6526f7a585a3").ok().unwrap();
        let expected_conditions = ConditionsBuilder::new(Duration::from_millis(100))
                                      .first_opens(true)
                                      .patterns(vec!["PATTERN_NAME1".to_string(),
                                                     "PATTERN_NAME2".to_string(),
                                                     "f13dafee-cd14-4dda-995c-6ed476a21de3"
                                                         .to_string()])
                                      .build();
        let message = Template::compile("message".to_string())
                          .ok()
                          .expect("Failed to compile a handlebars template");
        let expected_exec_cond = ExecCondition {
            on_opened: false,
            on_closed: true,
        };
        let expected_actions = vec![ActionType::Message(MessageActionBuilder::new("uuid1",
                                                                                  message)
                                                            .when(expected_exec_cond)
                                                            .build())];
        let context = result.ok().expect("Failed to deserialize a valid Context");
        assert_eq!(&Some(expected_name), &context.name);
        assert_eq!(&expected_uuid, &context.uuid);
        assert_eq!(&expected_conditions, &context.conditions);
        assert_eq!(&expected_actions, &context.actions);
    }

    #[test]
    fn test_given_config_context_when_it_does_not_have_uuid_then_it_cannot_be_deserialized() {
        let text = r#"{ "conditions": { "timeout": 100 }}"#;
        let result = from_str::<Context>(text);
        println!("{:?}", &result);
        let _ = result.err().expect("Successfully deserialized a config context without an uuid key");
    }

    #[test]
    fn test_given_config_context_when_it_contains_an_unknown_key_then_it_cannot_be_deserialized() {
        let text = r#"
            {"uuid": "86ca9f93-84fb-4813-b037-6526f7a585a3",
            "conditions": { "timeout": 100},
            "unknown": "unknown" }"#;
        let result = from_str::<Context>(text);
        println!("{:?}", &result);
        let _ = result.err().expect("Successfully deserialized a config context with an unknown key");
    }

    #[test]
    fn test_given_config_context_when_it_is_deserialized_and_only_the_required_fields_are_present_then_we_can_deserialize_it_successfully
        () {
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
        let expected_conditions = ConditionsBuilder::new(Duration::from_millis(100)).build();
        let context = result.ok().expect("Failed to deserialize a valid Context");
        assert_eq!(&expected_uuid, &context.uuid);
        assert_eq!(&expected_conditions, &context.conditions);
    }

    #[test]
    fn test_given_config_context_when_it_is_deserialized_and_the_uuid_is_invalid_then_we_report_an_error
        () {
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
        let _ = result.err()
                      .expect("Successfully deserialized an invalid Context (UUID is invalid)");
    }

    #[test]
    fn test_given_config_context_when_it_contains_context_id_then_can_be_deserialized() {
        let text = r#"
        {
            "uuid": "86ca9f93-84fb-4813-b037-6526f7a585a3",
            "context_id": "{{HOST}}{{PROGRAM}}",
            "conditions": {
                "timeout": 100
            }
        }
        "#;
        let expected_context_id = "{{HOST}}{{PROGRAM}}".to_string();
        let result = from_str::<Context>(text);
        println!("{:?}", &result);
        let context = result.ok().expect("Failed to deserialize a valid Context");
        assert_eq!(&expected_context_id,
                   &context.context_id.as_ref().unwrap().to_string());
    }
}
