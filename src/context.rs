use std::rc::Rc;

use super::{Message, Observer, TimerEvent};

#[derive(Clone)]
struct ConditionFields {
    timeout: u32,
    renew_timeout: Option<u32>,
    uuid: Option<String>,
    max_size: Option<usize>
}

impl ConditionFields {
    fn new(timeout: u32) -> ConditionFields {
        ConditionFields {
            timeout: timeout,
            renew_timeout: None,
            uuid: None,
            max_size: None
        }
    }
}

pub struct Context {
    conditions: ConditionFields,
    elapsed_time: u32,
    messages: Vec<Rc<Message>>
}

impl Context {
    fn is_max_size_reached(&self) -> bool {
        self.conditions.max_size.map_or(false, |max_size| self.messages.len() >= max_size)
    }

    fn is_closing_message(&self) -> bool {
        if let Some(event) = self.messages.last() {
            if let Some(uuid) = self.conditions.uuid.as_ref() {
                uuid == event.get("uuid").unwrap()
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_last_message(&self) -> bool {
       self.is_max_size_reached() || self.is_closing_message()
    }

    fn is_timer_expired(&self) -> bool {
        self.elapsed_time >= self.conditions.timeout
    }

    pub fn on_timer(&mut self, event: &TimerEvent) -> bool {
        println!("timer event: {}", event.0);
        self.elapsed_time += event.0;
        self.is_timer_expired()
    }

    pub fn on_message(&mut self, event: Rc<Message>) -> bool {
        println!("message event");
        self.messages.push(event);
        self.is_last_message()
    }
}

pub struct Builder {
    conditions: ConditionFields
}

impl Builder {
    pub fn new(timeout: u32) -> Builder {
        Builder{
            conditions: ConditionFields::new(timeout)
        }
    }

    pub fn renew_timeout(&mut self, timeout: u32) -> &mut Builder {
        self.conditions.renew_timeout = Some(timeout);
        self
    }

    pub fn uuid(&mut self, uuid: String) -> &mut Builder {
        self.conditions.uuid = Some(uuid);
        self
    }

    pub fn max_size(&mut self, max_size: usize) -> &mut Builder {
        self.conditions.max_size = Some(max_size);
        self
    }

    pub fn build(&mut self) -> Context {
        Context{
            messages: Vec::new(),
            conditions: self.conditions.clone(),
            elapsed_time: 0
        }
    }
}

#[test]
fn test_given_close_condition_with_timeout_when_the_timeout_expires_then_the_condition_is_met() {
    let timeout = 100;
    let mut condition = Builder::new(timeout).build();
    assert_false!(condition.on_timer(&mut TimerEvent(50)));
    assert_false!(condition.on_timer(&mut TimerEvent(49)));
    assert_true!(condition.on_timer(&mut TimerEvent(1)));
}

#[test]
fn test_given_close_condition_with_max_size_when_the_max_size_reached_then_the_condition_is_met() {
    let timeout = 100;
    let max_size = 3;
    let mut condition = Builder::new(timeout).max_size(max_size).build();
    let mut msg1 = btreemap!{
        "uuid".to_string() => "1".to_string(),
    };
    assert_false!(condition.on_message(&msg1));
    assert_false!(condition.on_message(&msg1));
    assert_true!(condition.on_message(&msg1));
}
