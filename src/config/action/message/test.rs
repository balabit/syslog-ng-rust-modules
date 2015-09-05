use message::Message;
use config::action::message::MessageActionBuilder;
use message::MessageBuilder;

#[test]
fn test_given_message_when_it_is_created_from_a_message_action_then_every_fields_are_copied_into_it() {
    let uuid = "uuid";
    let name = "name";
    let expected_message = MessageBuilder::new(uuid)
                                            .name(Some(name))
                                            .pair("key1", "value1")
                                            .pair("key2", "value1")
                                            .build();
    let action = MessageActionBuilder::new(uuid)
                                        .name(name)
                                        .pair("key1", "value1")
                                        .pair("key2", "value1")
                                        .build();
    let message = Message::from(&action);
    assert_eq!(&expected_message, &message);
}
