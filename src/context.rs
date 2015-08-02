use std::rc::Rc;

use super::{config, Conditions, Message, TimerEvent};

use self::linear::LinearContext;
use self::map::MapContext;

#[derive(Debug)]
pub enum Context {
    Linear(LinearContext),
    Map(MapContext)
}

impl Context {
    pub fn on_timer(&mut self, event: &TimerEvent) {
        match *self {
            Context::Linear(ref mut context) => context.on_timer(event),
            Context::Map(ref mut context) => context.on_timer(event),
        }
    }

    pub fn on_message(&mut self, event: Rc<Message>) {
        match *self {
            Context::Linear(ref mut context) => context.on_message(event),
            Context::Map(ref mut context) => context.on_message(event),
        }
    }

    pub fn is_open(&mut self) -> bool {
        match *self {
            Context::Linear(ref context) => context.is_open(),
            Context::Map(ref mut context) => context.is_open(),
        }
    }

    pub fn new_linear(conditions: Conditions) -> Context {
        Context::Linear(
            LinearContext::new(conditions)
        )
    }

    pub fn new_map(conditions: Conditions) -> Context {
        Context::Map(
            MapContext::new(conditions)
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
    use Message;
    use state::State;
    use TimerEvent;

    #[derive(Debug)]
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
            for (_, mut state) in self.map.iter_mut() {
                self.conditions.on_timer(event, &mut state);
            }
            self.remove_closed_states();
        }

        fn remove_closed_states(&mut self) {
            let ids_to_remove = self.map.iter().filter_map(|(id, state)| {
                if !state.is_open() {
                    Some(id.clone())
                } else {
                    None
                }
            }).collect::<Vec<String>>();

            for id in ids_to_remove {
                self.map.remove(&id);
            }
        }

        pub fn on_message(&mut self, event: Rc<Message>) {
            self.format_context_id(&event);
            self.update_state(event);
            self.format_buffer.clear();
        }

        fn update_state(&mut self, event: Rc<Message>) {
            let id = self.format_buffer.clone();

            match self.map.remove(&id) {
                Some(mut state) => {
                    self.conditions.on_message(event, &mut state);
                    if state.is_open() {
                        self.map.insert(id, state);
                    }
                },
                None => {
                    let mut state = State::new();
                    self.conditions.on_message(event, &mut state);
                    self.map.insert(id, state);
                }
            }
        }

        pub fn is_open(&mut self) -> bool {
            !self.map.is_empty()
        }

        fn format_context_id(&mut self, message: &Message) {
            let empty = String::new();
            let _ = self.format_buffer.write_str(message.get("HOST").unwrap_or(&empty));
            let _ = self.format_buffer.write_str(":");
            let _ = self.format_buffer.write_str(message.get("PROGRAM").unwrap_or(&empty));
            let _ = self.format_buffer.write_str(":");
            let _ = self.format_buffer.write_str(message.get("PID").unwrap_or(&empty));
        }
    }

    #[cfg(test)]
    mod test {
        use conditions::Builder;
        use Context;
        use TimerEvent;

        use std::rc::Rc;

        #[test]
        fn test_given_map_context_when_messages_have_the_same_kvpairs_then_they_go_to_the_same_context() {
            let delta = 10;
            let timeout = 30;
            let event = TimerEvent(delta);
            let patterns: Vec<String> = vec!["1".to_string(), "2".to_string(), "3".to_string()];
            let mut context = Context::new_map(Builder::new(timeout).patterns(patterns).build());
            let msg1 = Rc::new(btreemap! {
                "uuid".to_string() => "1".to_string(),
                "HOST".to_string() => "host".to_string(),
                "PROGRAM".to_string() => "program".to_string(),
                "PID".to_string() => "pid".to_string(),
            });
            let msg2 = Rc::new(btreemap! {
                "uuid".to_string() => "2".to_string(),
                "HOST".to_string() => "host2".to_string(),
                "PROGRAM".to_string() => "program2".to_string(),
                "PID".to_string() => "pid2".to_string(),
            });
            let msg3 = Rc::new(btreemap! {
                "uuid".to_string() => "3".to_string(),
                "HOST".to_string() => "host".to_string(),
                "PROGRAM".to_string() => "program".to_string(),
                "PID".to_string() => "pid".to_string(),
            });

            assert_false!(context.is_open());
            context.on_message(msg1.clone());
            assert_true!(context.is_open());
            context.on_timer(&event);
            context.on_message(msg2.clone());
            context.on_message(msg3.clone());
            context.on_timer(&event);
            context.on_timer(&event);
            assert_true!(context.is_open());
            context.on_timer(&event);
            assert_false!(context.is_open());
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use TimerEvent;
    use super::Context;
    use conditions::Builder;

    #[test]
    fn test_given_close_condition_with_timeout_when_the_timeout_expires_then_the_condition_is_met() {
        let timeout = 100;
        let msg_id = "1".to_string();
        let mut context = Context::new_linear(Builder::new(timeout).patterns(vec![msg_id.clone()]).build());
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
        let mut context = Context::new_linear(Builder::new(timeout).max_size(max_size).patterns(vec![msg_id.clone()]).build());
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
        let mut context = Context::new_linear(Builder::new(timeout).renew_timeout(renew_timeout).patterns(vec![msg_id.clone()]).build());
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
        let mut context = Context::new_linear(Builder::new(timeout).renew_timeout(renew_timeout).patterns(vec![msg_id.clone()]).build());
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
}
