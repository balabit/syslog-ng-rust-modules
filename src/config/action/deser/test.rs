use serde_json::from_str;
use config::action::ActionType;

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
    println!("{:?}", &result);
    let action = result.ok().expect("Failed to deserialize a valid ActionType");
    match action {
        ActionType::Message(message) => {
            assert_eq!("uuid1", message.uuid());
        }
    }
}
