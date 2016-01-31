use config::{ContextBuilder, Context};
use config::action::message::MessageActionBuilder;
use conditions::ConditionsBuilder;
use Correlator;
use message::MessageBuilder;


use handlebars::Template;
use uuid::Uuid;
use serde_json::from_str;
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;

use test_utils::correlator::MessageEventHandler;

const JSON_CONFIG: &'static str = r#"
      [
        {
          "name": "CONTEXT_NAME_1",
          "uuid": "185e96da-c00e-454b-b4fe-9d0a14a86335",
          "conditions": {
            "timeout": 100,
            "patterns": [
              "p1",
              "p2",
              "p3"
            ],
            "first_opens": true
          },
          "actions": [
            {
              "message": {
                  "uuid": "uuid1",
                  "when": {
                    "on_opened": "true",
                    "on_closed": "true"
                  },
                  "message": "message_1"
              }
            }
          ]
        },
        {
          "name": "CONTEXT_NAME_2",
          "uuid": "285e96da-c00e-454b-b4fe-9d0a14a86335",
          "conditions": {
            "timeout": 10000,
            "max_size": 5
          },
          "actions": [
            {
              "message": {
                  "uuid": "uuid1",
                  "message": "message_2"
              }
            },
            {
              "message": {
                  "uuid": "uuid2",
                  "message": "message_2"
              }
            }
          ]
        },
        {
          "name": "CONTEXT_NAME_3",
          "uuid": "385e96da-c00e-454b-b4fe-9d0a14a86335",
          "conditions": {
            "timeout": 100,
            "patterns": [
              "p1"
            ]
          },
          "actions": [
            {
              "message": {
                  "uuid": "uuid2",
                  "message": "message_3"
              }
            }
          ]
        }
      ]
    "#;

#[test]
fn test_given_manually_built_correlator_when_it_closes_a_context_then_the_actions_are_executed() {
    let uuid1 = "1b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let uuid2 = "2b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let uuid3 = "3b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let patterns = vec![
        uuid1.clone(),
        uuid2.clone(),
        uuid3.clone(),
    ];
    let condition = ConditionsBuilder::new(Duration::from_millis(100))
                        .patterns(patterns)
                        .first_opens(true)
                        .last_closes(true)
                        .build();
    let message = Template::compile("message".to_string())
                      .ok()
                      .expect("Failed to compile a handlebars template");
    let contexts = vec![
        ContextBuilder::new(Uuid::new_v4(), condition.clone()).actions(vec![MessageActionBuilder::new("uuid", message.clone()).build().into()]).build(),
        ContextBuilder::new(Uuid::new_v4(), condition.clone()).actions(vec![MessageActionBuilder::new("uuid", message.clone()).build().into()]).build(),
        ContextBuilder::new(Uuid::new_v4(), condition.clone()).actions(vec![MessageActionBuilder::new("uuid", message).build().into()]).build(),
    ];
    let responses = Rc::new(RefCell::new(Vec::new()));
    let message_event_handler = Box::new(MessageEventHandler { responses: responses.clone() });
    let mut correlator = Correlator::new(contexts);
    correlator.register_handler(message_event_handler);
    let _ = correlator.push_message(MessageBuilder::new(&uuid1, "message").build());
    thread::sleep(Duration::from_millis(20));
    let _ = correlator.push_message(MessageBuilder::new(&uuid2, "message").build());
    thread::sleep(Duration::from_millis(80));
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message").build());
    let _ = correlator.stop();
    assert_eq!(3, responses.borrow().len());
}

#[test]
fn test_given_correlator_when_it_is_built_from_json_then_we_get_the_expected_correlator() {
    let result = from_str::<Vec<Context>>(JSON_CONFIG);
    let expected_name = "CONTEXT_NAME_1".to_string();
    let expected_uuid = "185e96da-c00e-454b-b4fe-9d0a14a86335".to_string();
    let mut contexts = result.ok().expect("Failed to deserialize a config::Context from JSON");
    for i in &contexts {
        assert_eq!(true, i.name.is_some());
    }
    let context = contexts.remove(0);
    assert_eq!(Some(&expected_name), context.name.as_ref());
    assert_eq!(&expected_uuid, &context.uuid.to_hyphenated_string());
}

#[test]
fn test_given_correlator_when_it_is_built_from_json_then_it_produces_the_expected_number_of_messages
    () {
    let uuid1 = "1b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let uuid2 = "2b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let uuid3 = "3b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let result = from_str::<Vec<Context>>(JSON_CONFIG);
    let contexts = result.ok().expect("Failed to deserialize a config::Context from JSON");
    let responses = Rc::new(RefCell::new(Vec::new()));
    let message_event_handler = Box::new(MessageEventHandler { responses: responses.clone() });
    let mut correlator = Correlator::new(contexts);
    correlator.register_handler(message_event_handler);
    let _ = correlator.push_message(MessageBuilder::new(&uuid1, "message")
                                        .name(Some("p1"))
                                        .build());
    thread::sleep(Duration::from_millis(20));
    let _ = correlator.push_message(MessageBuilder::new(&uuid2, "message")
                                        .name(Some("p2"))
                                        .build());
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message")
                                        .name(Some("p3"))
                                        .build());
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message")
                                        .name(Some("p3"))
                                        .build());
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message")
                                        .name(Some("p3"))
                                        .build());
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message")
                                        .name(Some("p3"))
                                        .build());
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message")
                                        .name(Some("p3"))
                                        .build());
    thread::sleep(Duration::from_millis(200));
    let _ = correlator.stop();
    println!("{:?}", &responses.borrow());
    assert_eq!(5, responses.borrow().len());
}
