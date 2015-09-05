use super::MessageAction;

use serde::de::{
    Deserialize,
    Deserializer,
    Error,
    MapVisitor,
    Visitor
};
use std::collections::BTreeMap;

impl Deserialize for MessageAction {
    fn deserialize<D>(deserializer: &mut D) -> Result<MessageAction, D::Error>
        where D: Deserializer
    {
        deserializer.visit_struct("MessageAction", &[], MessageActionVisitor)
    }
}

enum Field {
    Uuid,
    Name,
    Values,
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
                    "values" => Ok(Field::Values),
                    _ => Err(Error::syntax(&format!("Unexpected field: {}", value))),
                }
            }
        }

        deserializer.visit(FieldVisitor)
    }
}

struct MessageActionVisitor;

impl Visitor for MessageActionVisitor {
    type Value = MessageAction;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<MessageAction, V::Error>
        where V: MapVisitor
    {
        let mut name = None;
        let mut uuid = None;
        let mut values = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Field::Name) => { name = Some(try!(visitor.visit_value())); }
                Some(Field::Uuid) => { uuid = Some(try!(visitor.visit_value())); }
                Some(Field::Values) => { values = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let uuid = match uuid {
            Some(uuid) => uuid,
            None => return visitor.missing_field("uuid"),
        };

        let values = match values {
            Some(values) => values,
            None => BTreeMap::new()
        };

        try!(visitor.end());

        Ok(
            MessageAction {
                name: name,
                uuid: uuid,
                values: values
            }
        )
    }
}

#[cfg(test)]
mod test {
    use config::action::message::{
        MessageActionBuilder, MessageAction
    };

    use serde_json::from_str;

    #[test]
    fn test_given_message_as_a_json_string_when_it_is_deserialized_then_we_get_the_expected_message() {
        let text = r#"
        {
          "uuid": "UUID",
          "name": "NAME",
          "values": {
            "key1": "value1",
            "key2": "value2"
          }
        }
        "#;

        let expected_message = MessageActionBuilder::new("UUID")
                                        .name("NAME")
                                        .pair("key1", "value1")
                                        .pair("key2", "value2")
                                        .build();
        let result = from_str::<MessageAction>(text);
        println!("{:?}", &result);
        let message = result.ok().expect("Failed to deserialize a valid MessageAction object");
        assert_eq!(expected_message, message);
    }

    #[test]
    fn test_given_message_as_a_json_string_when_only_the_required_fields_are_present_then_we_can_deserialize_the_message() {
        let text = r#"
        {
          "uuid": "UUID"
        }
        "#;

        let expected_message = MessageActionBuilder::new("UUID")
                                        .build();
        let result = from_str::<MessageAction>(text);
        println!("{:?}", &result);
        let message = result.ok().expect("Failed to deserialize a valid MessageAction object");
        assert_eq!(expected_message, message);
    }

    #[test]
    fn test_given_message_as_a_json_string_when_one_of_the_required_fields_are_not_present_then_we_get_error() {
        let text = r#"
        {
        }
        "#;

        let result = from_str::<MessageAction>(text);
        println!("{:?}", &result);
        let _ = result.err().expect("Successfully deserialized an invalid MessageAction object");
    }
}
