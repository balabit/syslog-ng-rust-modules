use uuid::Uuid;
use std::sync::Arc;
use std::time::Duration;

use message::MessageBuilder;
use timer::TimerEvent;
use context::LinearContext;
use conditions::ConditionsBuilder;

#[test]
fn test_given_close_condition_with_timeout_when_the_timeout_expires_then_the_condition_is_met() {
    let timeout = Duration::from_millis(100);
    let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
    let patterns = vec![
        msg_id.clone(),
    ];
    let mut context = LinearContext::new(Uuid::new_v4(),
                                         ConditionsBuilder::new(timeout)
                                             .patterns(patterns)
                                             .build());
    let msg1 = MessageBuilder::new(&msg_id, "message").build();
    let event = Arc::new(msg1);
    assert_false!(context.is_open());
    context.on_message(event);
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent::from_millis(50));
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent::from_millis(49));
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent::from_millis(1));
    assert_false!(context.is_open());
}

#[test]
fn test_given_close_condition_with_max_size_when_the_max_size_reached_then_the_condition_is_met
    () {
    let timeout = Duration::from_millis(100);
    let max_size = 3;
    let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
    let patterns = vec![
        msg_id.clone(),
    ];
    let mut context = LinearContext::new(Uuid::new_v4(),
                                         ConditionsBuilder::new(timeout)
                                             .max_size(max_size)
                                             .patterns(patterns)
                                             .build());
    let msg1 = MessageBuilder::new(&msg_id, "message").build();
    let event = Arc::new(msg1);
    context.on_message(event.clone());
    assert_true!(context.is_open());
    context.on_message(event.clone());
    assert_true!(context.is_open());
    context.on_message(event.clone());
    assert_false!(context.is_open());
}

#[test]
fn test_given_close_condition_with_renew_timeout_when_the_timeout_expires_without_renewing_messages_then_the_condition_is_met
    () {
    let timeout = Duration::from_millis(100);
    let renew_timeout = Duration::from_millis(10);
    let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
    let patterns = vec![
        msg_id.clone(),
    ];
    let mut context = LinearContext::new(Uuid::new_v4(),
                                         ConditionsBuilder::new(timeout)
                                             .renew_timeout(renew_timeout)
                                             .patterns(patterns)
                                             .build());
    let msg1 = MessageBuilder::new(&msg_id, "message").build();
    let event = Arc::new(msg1);
    context.on_message(event.clone());
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent::from_millis(8));
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent::from_millis(1));
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent::from_millis(1));
    assert_false!(context.is_open());
}

#[test]
fn test_given_close_condition_with_renew_timeout_when_the_timeout_expires_with_renewing_messages_then_the_context_is_not_closed
    () {
    let timeout = Duration::from_millis(100);
    let renew_timeout = Duration::from_millis(10);
    let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
    let patterns = vec![
        msg_id.clone(),
    ];
    let mut context = LinearContext::new(Uuid::new_v4(),
                                         ConditionsBuilder::new(timeout)
                                             .renew_timeout(renew_timeout)
                                             .patterns(patterns)
                                             .build());
    let msg1 = MessageBuilder::new(&msg_id, "message").build();
    let event = Arc::new(msg1);
    assert_false!(context.is_open());
    context.on_message(event.clone());
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent::from_millis(8));
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent::from_millis(1));
    assert_true!(context.is_open());
    context.on_message(event.clone());
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent::from_millis(1));
    assert_true!(context.is_open());
}
