use std::rc::Rc;

use state::State;
use super::{Action, config, Conditions, Message, TimerEvent};

use action::ExecResult;
use self::linear::LinearContext;
use self::map::MapContext;

#[derive(Debug)]
pub struct BaseContext {
    conditions: Conditions,
    actions: Vec<Action>
}

impl BaseContext {
    pub fn new(conditions: Conditions) -> BaseContext {
        BaseContext {
            conditions: conditions,
            actions: Vec::new()
        }
    }

    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    pub fn on_timer(&self, event: &TimerEvent, state: &mut State) -> Option<Vec<ExecResult>> {
        state.on_timer(event);
        if self.conditions.is_any_timer_expired(state) {
            println!("closing state");
            state.close(self)
        } else {
            None
        }
    }

    pub fn on_message(&self, event: Rc<Message>, state: &mut State) -> Option<Vec<ExecResult>> {
        if self.conditions.ignore_message(&event) {
            return None;
        }

        if state.is_open() {
            state.add_message(event);
            if self.conditions.is_closing(state) {
                return state.close(self);
            } else {
                return None;
            }
        } else if self.conditions.is_opening(&event) {
            state.add_message(event);
            state.open();
        }

        None
    }
}

impl From<config::Context> for BaseContext {
    fn from(config: config::Context) -> BaseContext {
        let config::Context{conditions, actions} = config;
        BaseContext {
            conditions: conditions,
            actions: actions
        }
    }
}

#[derive(Debug)]
pub enum Context {
    Linear(LinearContext),
    Map(MapContext)
}

impl Context {
    pub fn on_timer(&mut self, event: &TimerEvent) -> Option<Vec<ExecResult>> {
        match *self {
            Context::Linear(ref mut context) => context.on_timer(event),
            Context::Map(ref mut context) => context.on_timer(event),
        }
    }

    pub fn on_message(&mut self, event: Rc<Message>) -> Option<Vec<ExecResult>> {
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
    fn from(config: config::Context) -> Context {
        Context::Linear(LinearContext::from(config))
    }
}

mod linear {
    use std::rc::Rc;

    use action::ExecResult;
    use config;
    use Conditions;
    use Message;
    use state::State;
    use TimerEvent;
    use super::BaseContext;

    #[derive(Debug)]
    pub struct LinearContext {
        base: BaseContext,
        state: State
    }

    impl LinearContext {
        pub fn new(conditions: Conditions) -> LinearContext {
            LinearContext {
                base: BaseContext::new(conditions),
                state: State::new()
            }
        }

        pub fn on_timer(&mut self, event: &TimerEvent) -> Option<Vec<ExecResult>> {
            self.base.on_timer(event, &mut self.state)
        }

        pub fn on_message(&mut self, event: Rc<Message>) -> Option<Vec<ExecResult>> {
            self.base.on_message(event, &mut self.state)
        }

        pub fn is_open(&self) -> bool {
            self.state.is_open()
        }
    }

    impl From<config::Context> for LinearContext {
        fn from(config: config::Context) -> LinearContext {
            LinearContext {
                base: BaseContext::from(config),
                state: State::new()
            }
        }
    }
}

mod map {
    use std::collections::BTreeMap;
    use std::fmt::Write;
    use std::rc::Rc;

    use action::ExecResult;
    use Conditions;
    use Message;
    use state::State;
    use TimerEvent;
    use super::BaseContext;

    #[derive(Debug)]
    pub struct MapContext {
        base: BaseContext,
        map: BTreeMap<String, State>,
        format_buffer: String
    }

    impl MapContext {
        pub fn new(conditions: Conditions) -> MapContext {
            MapContext {
                base: BaseContext::new(conditions),
                map: BTreeMap::new(),
                format_buffer: String::new()
            }
        }

        pub fn on_timer(&mut self, event: &TimerEvent) -> Option<Vec<ExecResult>> {
            let mut result: Vec<ExecResult> = Vec::new();

            for (_, mut state) in self.map.iter_mut() {
                if let Some(commands) = self.base.on_timer(event, &mut state) {
                    for i in commands.into_iter() {
                        result.push(i);
                    }
                }
            }

            self.remove_closed_states();
            if result.is_empty() {
                None
            } else {
                Some(result)
            }
        }

        fn get_closed_state_ids(&self) -> Vec<String> {
            self.map.iter().filter_map(|(id, state)| {
                if !state.is_open() {
                    Some(id.clone())
                } else {
                    None
                }
            }).collect::<Vec<String>>()
        }

        fn remove_closed_states(&mut self) {
            for id in self.get_closed_state_ids() {
                let _ = self.map.remove(&id);
            }
        }

        pub fn on_message(&mut self, event: Rc<Message>) -> Option<Vec<ExecResult>> {
            self.format_context_id(&event);
            let result = self.update_state(event);
            self.format_buffer.clear();
            self.remove_closed_states();
            result
        }

        fn update_state(&mut self, event: Rc<Message>) -> Option<Vec<ExecResult>> {
            let id = self.format_buffer.clone();
            let state = self.map.entry(id).or_insert(State::new());
            self.base.on_message(event, state)
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
        use message;

        use std::rc::Rc;

        #[test]
        fn test_given_map_context_when_messages_have_the_same_kvpairs_then_they_go_to_the_same_context() {
            let delta = 10;
            let timeout = 30;
            let event = TimerEvent(delta);
            let patterns: Vec<String> = vec!["1".to_string(), "2".to_string(), "3".to_string()];
            let mut context = Context::new_map(Builder::new(timeout).patterns(patterns).build());
            let msg1 = message::Builder::new("1".to_string())
                                        .pair("HOST".to_string(), "host".to_string())
                                        .pair("PROGRAM".to_string(), "program".to_string())
                                        .pair("PID".to_string(), "pid".to_string())
                                        .build();
            let msg2 = message::Builder::new("2".to_string())
                                        .pair("HOST".to_string(), "host2".to_string())
                                        .pair("PROGRAM".to_string(), "program2".to_string())
                                        .pair("PID".to_string(), "pid2".to_string())
                                        .build();
            let msg3 = message::Builder::new("3".to_string())
                                        .pair("HOST".to_string(), "host".to_string())
                                        .pair("PROGRAM".to_string(), "program".to_string())
                                        .pair("PID".to_string(), "pid".to_string())
                                        .build();

            assert_false!(context.is_open());
            context.on_message(Rc::new(msg1));
            assert_true!(context.is_open());
            context.on_timer(&event);
            context.on_message(Rc::new(msg2));
            context.on_message(Rc::new(msg3));
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

    use message;
    use TimerEvent;
    use super::Context;
    use conditions::Builder;

    #[test]
    fn test_given_close_condition_with_timeout_when_the_timeout_expires_then_the_condition_is_met() {
        let timeout = 100;
        let msg_id = "1".to_string();
        let mut context = Context::new_linear(Builder::new(timeout).patterns(vec![msg_id.clone()]).build());
        let msg1 = message::Builder::new(msg_id.clone()).build();
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
        let msg1 = message::Builder::new(msg_id.clone()).build();
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
        let msg1 = message::Builder::new(msg_id.clone()).build();
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
        let msg1 = message::Builder::new(msg_id.clone()).build();
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
