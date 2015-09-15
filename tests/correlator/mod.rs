use correlation::Correlator;
use correlation::message::MessageBuilder;
use correlation::test_utils::correlator::MessageEventHandler;

use env_logger;

use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_given_correlator_when_messages_are_received_then_they_are_grouped_into_a_context_by_a_context_id() {
    let _ = env_logger::init();
    let contexts_file = "tests/correlator/contexts.json";
    let mut correlator = Correlator::from_path(contexts_file).ok().expect("Failed to load contexts from a valid contexts_file");
    let login_message = MessageBuilder::new("6d2cba0c-e241-464a-89c3-8035cac8f73e", "message")
                                        .name(Some("LOGIN"))
                                        .pair("user_name", "linus")
                                        .build();
    let read_message = MessageBuilder::new("60dd1233-5fa6-4e3b-993f-e04ef9b4c164", "message")
                                        .name(Some("MAIL_READ"))
                                        .pair("user_name", "linus")
                                        .build();
    let logout_message = MessageBuilder::new("91ea534a-4880-4853-aec2-7b2a2df9a8c9", "message")
                                        .name(Some("LOGOUT"))
                                        .pair("user_name", "linus")
                                        .build();
    let responses = Rc::new(RefCell::new(Vec::new()));
    let message_event_handler = Box::new(MessageEventHandler{responses: responses.clone()});
    correlator.register_handler(message_event_handler);
    let _ = correlator.push_message(login_message);
    let _ = correlator.push_message(read_message);
    let _ = correlator.push_message(logout_message);
    correlator.handle_events();
    correlator.stop();
    assert_eq!(1, responses.borrow().len());
}
