use super::MessageAction;

use handlebars::Template;
use serde::de::{Deserialize, Deserializer, Error, MapVisitor, Visitor};
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
    Message,
    Values,
    OnOpened,
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
                    "message" => Ok(Field::Message),
                    "on_opened" => Ok(Field::OnOpened),
                    _ => Err(Error::syntax(&format!("Unexpected field: {}", value))),
                }
            }
        }

        deserializer.visit(FieldVisitor)
    }
}

struct MessageActionVisitor;

impl MessageActionVisitor {
    fn compile_template<V>(template_string: String, uuid: &String) -> Result<Template, V::Error>
        where V: MapVisitor
    {
        match Template::compile(template_string) {
            Ok(message) => Ok(message),
            Err(error) => {
                return Err(Error::syntax(&format!("Invalid handlebars template in 'message' \
                                                   field: uuid={}, error={}",
                                                  &uuid,
                                                  error)));
            }
        }
    }
}

impl Visitor for MessageActionVisitor {
    type Value = MessageAction;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<MessageAction, V::Error>
        where V: MapVisitor
    {
        let mut name = None;
        let mut uuid = None;
        let mut message: Option<String> = None;
        let mut values: Option<BTreeMap<String, String>> = None;
        let mut on_opened: Option<bool> = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Field::Name) => {
                    name = Some(try!(visitor.visit_value()));
                }
                Some(Field::Uuid) => {
                    uuid = Some(try!(visitor.visit_value()));
                }
                Some(Field::Message) => {
                    message = Some(try!(visitor.visit_value()));
                }
                Some(Field::Values) => {
                    values = Some(try!(visitor.visit_value()));
                }
                Some(Field::OnOpened) => {
                    on_opened = Some(try!(visitor.visit_value()));
                }
                None => {
                    break;
                }
            }
        }

        let uuid = match uuid {
            Some(uuid) => uuid,
            None => return visitor.missing_field("uuid"),
        };

        let message = match message {
            Some(message) => {
                try!(MessageActionVisitor::compile_template::<V>(message, &uuid))
            }
            None => {
                error!("Missing 'message' field: uuid={}", &uuid);
                return visitor.missing_field("message");
            }
        };

        let values = match values {
            Some(values) => {
                let mut converted_values = BTreeMap::new();
                for (key, value) in values.into_iter() {
                    let template = try!(MessageActionVisitor::compile_template::<V>(value, &uuid));
                    converted_values.insert(key, template);
                }
                converted_values
            }
            None => BTreeMap::new(),
        };

        try!(visitor.end());

        Ok(MessageAction {
            name: name,
            uuid: uuid,
            message: message,
            values: values,
            on_opened: on_opened
        })
    }
}

#[cfg(test)]
mod test {
    use config::action::message::{MessageActionBuilder, MessageAction};

    use handlebars::Template;
    use serde_json::from_str;

    #[test]
    fn test_given_message_as_a_json_string_when_it_is_deserialized_then_we_get_the_expected_message
        () {
        let text = r#"
        {
          "uuid": "UUID",
          "name": "NAME",
          "message": "message",
          "values": {
            "key1": "value1",
            "key2": "value2"
          }
        }
        "#;

        let message = Template::compile("message".to_string())
                          .ok()
                          .expect("Failed to compile a handlebars template");
        let value1 = Template::compile("value1".to_string())
                         .ok()
                         .expect("Failed to compile a handlebars template");
        let value2 = Template::compile("value2".to_string())
                         .ok()
                         .expect("Failed to compile a handlebars template");
        let expected_message = MessageActionBuilder::new("UUID", message)
                                   .name("NAME")
                                   .pair("key1", value1)
                                   .pair("key2", value2)
                                   .build();
        let result = from_str::<MessageAction>(text);
        println!("{:?}", &result);
        let message = result.ok().expect("Failed to deserialize a valid MessageAction object");
        assert_eq!(expected_message, message);
    }

    #[test]
    fn test_given_message_as_a_json_string_when_only_the_required_fields_are_present_then_we_can_deserialize_the_message
        () {
        let text = r#"
        {
          "uuid": "UUID",
          "message": "message"
        }
        "#;

        let message = Template::compile("message".to_string())
                          .ok()
                          .expect("Failed to compile a handlebars template");
        let expected_message = MessageActionBuilder::new("UUID", message).build();
        let result = from_str::<MessageAction>(text);
        println!("{:?}", &result);
        let message = result.ok().expect("Failed to deserialize a valid MessageAction object");
        assert_eq!(expected_message, message);
    }

    #[test]
    fn test_given_message_as_a_json_string_when_one_of_the_required_fields_are_not_present_then_we_get_error
        () {
        let text = r#"
        {
        }
        "#;

        let result = from_str::<MessageAction>(text);
        println!("{:?}", &result);
        let _ = result.err().expect("Successfully deserialized an invalid MessageAction object");
    }
}
