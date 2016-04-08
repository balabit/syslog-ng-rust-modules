// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use config::action::message::MessageActionBuilder;
use message::MessageBuilder;
use context::base::BaseContextBuilder;

use conditions::ConditionsBuilder;
use dispatcher::Response;
use test_utils::MockResponseSender;
use state::State;
use action::Action;

use env_logger;
use std::time::Duration;
use std::sync::Arc;
use uuid::Uuid;
use Event;

#[test]
fn test_given_message_action_when_it_is_executed_then_the_additional_values_are_inserted_into_the_generated_message
    () {
    let mut responder = MockResponseSender::default();
    let _ = env_logger::init();
    let base_context = {
        let conditions = ConditionsBuilder::new(Duration::from_millis(100)).build();
        let uuid = Uuid::new_v4();
        BaseContextBuilder::new(uuid, conditions).name(Some("name".to_owned())).build()
    };
    let state = {
        let messages = vec![Arc::new(MessageBuilder::new("uuid1", "message1").build()),
                            Arc::new(MessageBuilder::new("uuid2", "message2").build())];
        State::with_messages(messages)
    };
    let message_action = MessageActionBuilder::new("uuid", "message")
                                              .pair("key1", "value1")
                                              .pair("key2", "value2")
                                              .build();

    message_action.on_closed(&state, &base_context, &mut responder);
    assert_eq!(1, responder.0.len());
    let responses = responder.0;
    if let Response::Alert(ref response) = *responses.get(0).unwrap() {
        let message = &response.message;
        assert_eq!("value1",
                   message.get("key1").expect("Failed to get an additional key-value pair from a generated message"));
        assert_eq!("value2",
                   message.get("key2").expect("Failed to get an additional key-value pair from a generated message"));
    } else {
        unreachable!();
    }
}
