use std::rc::Rc;

use super::{config, Conditions, Message, TimerEvent};
use state::State;

use self::linear::LinearContext;

#[derive(Debug)]
pub enum Context {
    Linear(LinearContext),
}

impl Context {
    pub fn on_timer(&mut self, event: &TimerEvent) {
        match *self {
            Context::Linear(ref mut context) => context.on_timer(event)
        }
    }

    pub fn on_message(&mut self, event: Rc<Message>) {
        match *self {
            Context::Linear(ref mut context) => context.on_message(event)
        }
    }

    pub fn is_open(&self) -> bool {
        match *self {
            Context::Linear(ref context) => context.is_open()
        }
    }

    pub fn new_linear(conditions: Conditions) -> Context {
        Context::Linear(
            LinearContext::new(conditions)
        )
    }
}

impl From<config::Context> for Context {
    fn from(context: config::Context) -> Context {
        Context::new_linear(context.conditions)
    }
}

mod linear {
    use std::rc::Rc;

    use Conditions;
    use config;
    use Message;
    use state::State;
    use TimerEvent;

    #[derive(Debug)]
    pub struct LinearContext {
        conditions: Conditions,
        state: State
    }

    impl LinearContext {
        pub fn new(conditions: Conditions) -> LinearContext {
            LinearContext {
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
}

mod map {
    use std::collections::BTreeMap;
    use std::fmt::Write;
    use std::rc::Rc;

    use Conditions;
    use config;
    use Message;
    use state::State;
    use TimerEvent;

    pub struct MapContext {
        map: BTreeMap<String, State>,
        conditions: Conditions,
        format_buffer: String
    }

    impl MapContext {
        pub fn new(conditions: Conditions) -> MapContext {
            MapContext {
                map: BTreeMap::new(),
                conditions: conditions,
                format_buffer: String::new()
            }
        }

        pub fn on_timer(&mut self, event: &TimerEvent) {
            for (id, mut state) in self.map.iter_mut() {
                self.conditions.on_timer(event, &mut state);
            }
        }

        pub fn on_message(&mut self, event: Rc<Message>) {
            self.format_context_id(&event);
            let state = self.map.entry(self.format_buffer.clone()).or_insert(State::new());
            self.conditions.on_message(event, state);
            self.format_buffer.clear();
        }

        pub fn is_open(&mut self) -> bool {
            !self.map.is_empty()
        }

        fn format_context_id(&mut self, message: &Message) {
            let _ = self.format_buffer.write_str("foo bar baz");
        }
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
