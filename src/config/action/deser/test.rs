use serde_json::from_str;
use config::action::{ActionType, ExecCondition};

#[test]
fn test_given_action_when_it_is_deserialized_then_we_get_the_right_result() {
    let text = r#"
        {
            "message": {
                "uuid": "uuid1",
                "message": "message"
            }
        }
    "#;

    let result = from_str::<ActionType>(text);
    let action = result.ok().expect("Failed to deserialize a valid ActionType");
    match action {
        ActionType::Message(message) => {
            assert_eq!("uuid1", message.uuid());
        }
    }
}

#[test]
fn test_given_unknown_action_when_it_is_deserialized_then_we_get_an_error() {
    let text = r#"{ "unknown": {} }"#;
    let result = from_str::<ActionType>(text);
    let _ = result.err().expect("Successfully deserialized an unknown action");
}

#[test]
fn test_given_filled_exec_condition_when_it_is_deserialized_then_it_is_populated_with_the_specified_values
    () {
    let text = r#"
        {
            "on_opened": true,
            "on_closed": false
        }
    "#;

    let expected = ExecCondition {
        on_closed: false,
        on_opened: true,
    };
    let result = from_str::<ExecCondition>(text);
    println!("{:?}", &result);
    let cond = result.ok().expect("Failed to deserialize a valid ExecCondition");
    assert_eq!(expected, cond);
}

#[test]
fn test_given_exec_condition_when_it_contains_an_unknown_key_then_the_deserialization_fails() {
    let text = r#"{ "unknown": true }"#;
    let result = from_str::<ExecCondition>(text);
    println!("{:?}", &result);
    let _ = result.err().expect("Failed to deserialize a valid ExecCondition");
}

#[test]
fn test_given_filled_exec_condition_when_it_is_deserialized_then_its_missing_fields_are_populated_with_default_values
    () {
    let text = r#"
        {
        }
    "#;

    let expected: ExecCondition = Default::default();
    let result = from_str::<ExecCondition>(text);
    println!("{:?}", &result);
    let cond = result.ok().expect("Failed to deserialize a valid ExecCondition");
    assert_eq!(expected, cond);
}
