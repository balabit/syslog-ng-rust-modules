use context::base;
use super::{
    CONTEXT_NAME,
    CONTEXT_UUID,
    MessageAction,
};

use action::Action;
use conditions;
use config;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use std::cell::RefCell;
use std::rc::Rc;
use state::State;

use uuid::Uuid;

struct DummyResponseSender {
    responses: Rc<RefCell<Vec<Response>>>
}

impl ResponseSender<Response> for DummyResponseSender {
    fn send_response(&mut self, response: Response) {
        self.responses.borrow_mut().push(response);
    }
}

#[test]
fn test_given_a_message_action_when_it_is_executed_then_it_adds_the_name_and_uuid_of_the_context_to_the_message() {
    let name = Some("name".to_string());
    let base_context = {
        let conditions = conditions::Builder::new(100).build();
        let uuid = Uuid::new_v4();
        base::Builder::new(uuid, conditions).name(name.clone()).build()
    };
    let state = State::new();
    let responses = Rc::new(RefCell::new(Vec::new()));
    let message_action = {
        let response_sender = DummyResponseSender {responses: responses.clone()};
        let config_action = config::action::message::MessageActionBuilder::new("uuid").build();
        MessageAction {
            sender: Rc::new(RefCell::new(Box::new(response_sender))),
            action: config_action
        }
    };

    message_action.execute(&state, &base_context);
    assert_eq!(1, responses.borrow().len());
    let responses = responses.borrow();
    if let &Response::Message(ref response) = responses.get(0).unwrap() {
        assert_eq!(name.as_ref().unwrap(), response.message().get(CONTEXT_NAME).unwrap());
        assert_eq!(&base_context.uuid().to_hyphenated_string(), response.message().get(CONTEXT_UUID).unwrap());
    } else {
        unreachable!();
    }
}
