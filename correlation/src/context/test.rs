// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use uuid::Uuid;
use std::sync::Arc;
use std::time::Duration;

use message::MessageBuilder;
use timer::TimerEvent;
use context::LinearContext;
use context::BaseContextBuilder;
use conditions::ConditionsBuilder;
use test_utils::{MockResponseSender, MockTemplate};
use Message;

#[test]
fn test_given_close_condition_with_timeout_when_the_timeout_expires_then_the_condition_is_met() {
    let mut responder = MockResponseSender::default();
    let timeout = Duration::from_millis(100);
    let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_owned();
    let patterns = vec![
        msg_id.clone(),
    ];
    let conditions = ConditionsBuilder::new(timeout)
                         .build();
    let base = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), conditions).patterns(patterns).build();
    let mut context = LinearContext::new(base);
    let msg1 = MessageBuilder::new(&msg_id, "message").build();
    let event = Arc::new(msg1);
    assert_false!(context.is_open());
    context.on_message(event, &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&TimerEvent::from_millis(50), &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&TimerEvent::from_millis(49), &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&TimerEvent::from_millis(1), &mut responder);
    assert_false!(context.is_open());
}

#[test]
fn test_given_close_condition_with_max_size_when_the_max_size_reached_then_the_condition_is_met
    () {
    let mut responder = MockResponseSender::default();
    let timeout = Duration::from_millis(100);
    let max_size = 3;
    let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_owned();
    let patterns = vec![
        msg_id.clone(),
    ];
    let conditions = ConditionsBuilder::new(timeout)
                         .max_size(max_size)
                         .build();
    let base = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), conditions).patterns(patterns).build();
    let mut context = LinearContext::new(base);
    let msg1 = MessageBuilder::new(&msg_id, "message").build();
    let event = Arc::new(msg1);
    context.on_message(event.clone(), &mut responder);
    assert_true!(context.is_open());
    context.on_message(event.clone(), &mut responder);
    assert_true!(context.is_open());
    context.on_message(event.clone(), &mut responder);
    assert_false!(context.is_open());
}

#[test]
fn test_given_close_condition_with_renew_timeout_when_the_timeout_expires_without_renewing_messages_then_the_condition_is_met
    () {
    let mut responder = MockResponseSender::default();
    let timeout = Duration::from_millis(100);
    let renew_timeout = Duration::from_millis(10);
    let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_owned();
    let patterns = vec![
        msg_id.clone(),
    ];
    let conditions = ConditionsBuilder::new(timeout)
                         .renew_timeout(renew_timeout)
                         .build();
    let base = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), conditions).patterns(patterns).build();
    let mut context = LinearContext::new(base);
    let msg1 = MessageBuilder::new(&msg_id, "message").build();
    let event = Arc::new(msg1);
    context.on_message(event.clone(), &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&TimerEvent::from_millis(8), &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&TimerEvent::from_millis(1), &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&TimerEvent::from_millis(1), &mut responder);
    assert_false!(context.is_open());
}

#[test]
fn test_given_close_condition_with_renew_timeout_when_the_timeout_expires_with_renewing_messages_then_the_context_is_not_closed
    () {
    let mut responder = MockResponseSender::default();
    let timeout = Duration::from_millis(100);
    let renew_timeout = Duration::from_millis(10);
    let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_owned();
    let patterns = vec![
        msg_id.clone(),
    ];
    let conditions = ConditionsBuilder::new(timeout)
                         .renew_timeout(renew_timeout)
                         .build();
    let base = BaseContextBuilder::<Message, MockTemplate>::new(Uuid::new_v4(), conditions).patterns(patterns).build();
    let mut context = LinearContext::new(base);
    let msg1 = MessageBuilder::new(&msg_id, "message").build();
    let event = Arc::new(msg1);
    assert_false!(context.is_open());
    context.on_message(event.clone(), &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&TimerEvent::from_millis(8), &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&TimerEvent::from_millis(1), &mut responder);
    assert_true!(context.is_open());
    context.on_message(event.clone(), &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&TimerEvent::from_millis(1), &mut responder);
    assert_true!(context.is_open());
}
