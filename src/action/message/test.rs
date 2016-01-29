use context::base::BaseContextBuilder;
use super::{CONTEXT_LEN, CONTEXT_NAME, CONTEXT_UUID, MessageAction};

use conditions::ConditionsBuilder;
use config;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use message::MessageBuilder;
use state::State;

use env_logger;
use handlebars::Template;
use std::cell::RefCell;
use std::time::Duration;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone)]
struct DummyResponseSender {
    responses: Rc<RefCell<Vec<Response>>>,
}

impl ResponseSender<Response> for DummyResponseSender {
    fn send_response(&self, response: Response) {
        self.responses.borrow_mut().push(response);
    }

    fn boxed_clone(&self) -> Box<ResponseSender<Response>> {
        Box::new(self.clone())
    }
}

#[test]
fn test_given_dummy_response_handler_can_be_cloned() {
    let responses = Rc::new(RefCell::new(Vec::new()));
    let response_sender = DummyResponseSender { responses: responses.clone() };
    let _ = response_sender.boxed_clone();
}

#[test]
fn test_given_a_message_action_when_it_is_executed_then_it_adds_the_name_and_uuid_of_the_context_to_the_message
    () {
    let name = Some("name".to_string());
    let base_context = {
        let conditions = ConditionsBuilder::new(Duration::from_millis(100)).build();
        let uuid = Uuid::new_v4();
        BaseContextBuilder::new(uuid, conditions).name(name.clone()).build()
    };
    let state = State::new();
    let responses = Rc::new(RefCell::new(Vec::new()));
    let message_action = {
        let response_sender = DummyResponseSender { responses: responses.clone() };
        let message = Template::compile("message".to_string())
                          .ok()
                          .expect("Failed to compile a handlebars template");
        let config_action = config::action::message::MessageActionBuilder::new("uuid", message)
                                .build();
        MessageAction::new(Box::new(response_sender),
                           config_action)
    };

    message_action.execute(&state, &base_context);
    assert_eq!(1, responses.borrow().len());
    let responses = responses.borrow();
    if let &Response::Alert(ref response) = responses.get(0).unwrap() {
        assert_eq!(name.as_ref().unwrap(),
                   response.message().get(CONTEXT_NAME).unwrap());
        assert_eq!(&base_context.uuid().to_hyphenated_string(),
                   response.message().get(CONTEXT_UUID).unwrap());
        assert_eq!("0", response.message().get(CONTEXT_LEN).unwrap());
    } else {
        unreachable!();
    }
}

#[test]
fn test_given_message_action_when_it_is_executed_then_it_uses_the_messages_to_render_the_message_and_additonal_templated_values
    () {
    let _ = env_logger::init();
    let name = Some("name".to_string());
    let base_context = {
        let conditions = ConditionsBuilder::new(Duration::from_millis(100)).build();
        let uuid = Uuid::new_v4();
        BaseContextBuilder::new(uuid, conditions).name(name.clone()).build()
    };
    let state = {
        let messages = vec![
            Rc::new(MessageBuilder::new("uuid1", "message1")
                                      .pair("key1", "value1")
                                      .build()),
            Rc::new(MessageBuilder::new("uuid2", "message2")
                                      .pair("key2", "value2")
                                      .build())
        ];
        State::with_messages(messages)
    };
    let responses = Rc::new(RefCell::new(Vec::new()));
    let message_action = {
        let response_sender = DummyResponseSender { responses: responses.clone() };
        let message = Template::compile("key1={{{messages.[0].values.key1}}} \
                                         key2={{{messages.[1].values.key2}}}"
                                            .to_string())
                          .ok()
                          .expect("Failed to compile a handlebars template");
        let config_action = config::action::message::MessageActionBuilder::new("uuid", message)
                                .pair("message_num",
                                      Template::compile("we have {{context_len}} messages"
                                                            .to_string())
                                          .ok()
                                          .expect("Failed to compile a handlebars template"))
                                .build();
        MessageAction::new(Box::new(response_sender),
                           config_action)
    };

    message_action.execute(&state, &base_context);
    assert_eq!(1, responses.borrow().len());
    let responses = responses.borrow();
    if let &Response::Alert(ref response) = responses.get(0).unwrap() {
        assert_eq!(name.as_ref().unwrap(),
                   response.message().get(CONTEXT_NAME).unwrap());
        assert_eq!(&base_context.uuid().to_hyphenated_string(),
                   response.message().get(CONTEXT_UUID).unwrap());
        let message = response.message();
        assert_eq!("we have 2 messages",
                   message.get("message_num").expect("Failed to get an inserted key from a map"));
        assert_eq!("key1=value1 key2=value2", message.message());
    } else {
        unreachable!();
    }
}
