use config;
use conditions;
use Correlator;
use dispatcher::ResponseHandler;
use message;
use Response;

use uuid::Uuid;
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;

use reactor::EventHandler;
use action::MessageResponse;

struct MessageEventHandler{
    responses: Rc<RefCell<Vec<MessageResponse>>>
}

impl EventHandler<Response> for MessageEventHandler {
    fn handle_event(&mut self, event: Response) {
        if let Response::Message(event) = event {
            self.responses.borrow_mut().push(event);
        }
    }
    fn handler(&self) -> ResponseHandler {
        ResponseHandler::Message
    }
}

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
    let condition = conditions::Builder::new(100).patterns(patterns)
                                                .first_opens(true)
                                                .last_closes(true)
                                                .build();
    let actions = vec![ config::action::MessageAction.into() ];
    let contexts = vec!{
        config::ContextBuilder::new(Uuid::new_v4(), condition.clone()).actions(actions.clone()).build(),
        config::ContextBuilder::new(Uuid::new_v4(), condition.clone()).actions(actions.clone()).build(),
        config::ContextBuilder::new(Uuid::new_v4(), condition.clone()).actions(actions.clone()).build(),
    };
    let responses = Rc::new(RefCell::new(Vec::new()));
    let message_event_handler = Box::new(MessageEventHandler{responses: responses.clone()});
    let mut correlator = Correlator::new(contexts);
    correlator.register_handler(message_event_handler);
    let _ = correlator.push_message(message::Builder::new(&uuid1).build());
    thread::sleep_ms(20);
    let _ = correlator.push_message(message::Builder::new(&uuid2).build());
    thread::sleep_ms(80);
    let _ = correlator.push_message(message::Builder::new(&uuid3).build());
    let _ = correlator.stop();
    assert_eq!(3, responses.borrow().len());
}
