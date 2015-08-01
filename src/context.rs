use std::rc::Rc;

use super::{config, Conditions, Message, TimerEvent};
use state::State;

#[derive(Debug)]
pub struct Context {
    conditions: Conditions,
    state: State
}

impl Context {
    pub fn new(conditions: Conditions) -> Context {
        Context {
            conditions: conditions,
            state: State::new()
        }
    }

    pub fn on_timer(&mut self, event: &TimerEvent) {
        self.conditions.on_timer(event, &mut self.state);
    }

    pub fn on_message(&mut self, event: Rc<Message>) {
        self.conditions.on_message(event, &mut self.state);
    }

    pub fn is_open(&self) -> bool {
        self.state.is_open()
    }
}

impl From<config::Context> for Context {
    fn from(context: config::Context) -> Context {
        Context::new(context.conditions)
    }
}

use conditions::Builder;

#[test]
fn test_given_close_condition_with_timeout_when_the_timeout_expires_then_the_condition_is_met() {
    let timeout = 100;
    let msg_id = "1".to_string();
    let mut context = Context::new(Builder::new(timeout).patterns(vec![msg_id.clone()]).build());
    let msg1 = btreemap!{
        "uuid".to_string() => msg_id.clone(),
    };
    let event = Rc::new(msg1);
    println!("{:?}", &context);
    assert_false!(context.is_open());
    context.on_message(event);
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent(50));
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent(49));
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent(1));
    assert_false!(context.is_open());
}
#[test]
fn test_given_close_condition_with_max_size_when_the_max_size_reached_then_the_condition_is_met() {
    let timeout = 100;
    let max_size = 3;
    let msg_id = "1".to_string();
    let mut context = Context::new(Builder::new(timeout).max_size(max_size).patterns(vec![msg_id.clone()]).build());
    let msg1 = btreemap!{
        "uuid".to_string() => msg_id.clone(),
    };
    let event = Rc::new(msg1);
    println!("{:?}", &context);
    context.on_message(event.clone());
    assert_true!(context.is_open());
    context.on_message(event.clone());
    assert_true!(context.is_open());
    context.on_message(event.clone());
    println!("{:?}", &context);
    assert_false!(context.is_open());
}

#[test]
fn test_given_close_condition_with_renew_timeout_when_the_timeout_expires_without_renewing_messages_then_the_condition_is_met() {
    let timeout = 100;
    let renew_timeout = 10;
    let msg_id = "1".to_string();
    let mut context = Context::new(Builder::new(timeout).renew_timeout(renew_timeout).patterns(vec![msg_id.clone()]).build());
    let msg1 = btreemap!{
        "uuid".to_string() => msg_id.clone(),
    };
    let event = Rc::new(msg1);
    context.on_message(event.clone());
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent(8));
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent(1));
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent(1));
    assert_false!(context.is_open());
}

#[test]
fn test_given_close_condition_with_renew_timeout_when_the_timeout_expires_with_renewing_messages_then_the_context_is_not_closed() {
    let timeout = 100;
    let renew_timeout = 10;
    let msg_id = "1".to_string();
    let mut context = Context::new(Builder::new(timeout).renew_timeout(renew_timeout).patterns(vec![msg_id.clone()]).build());
    let msg1 = btreemap!{
        "uuid".to_string() => msg_id.clone(),
    };
    let event = Rc::new(msg1);
    assert_false!(context.is_open());
    context.on_message(event.clone());
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent(8));
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent(1));
    assert_true!(context.is_open());
    context.on_message(event.clone());
    assert_true!(context.is_open());
    context.on_timer(&mut TimerEvent(1));
    assert_true!(context.is_open());
}
