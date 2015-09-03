use dispatcher::request::InternalRequest;

pub use self::linear::LinearContext;
pub use self::map::MapContext;

pub mod base;
pub mod event;
pub mod map;

pub enum Context {
    Linear(LinearContext),
    Map(MapContext)
}

impl From<Context> for Box<self::event::EventHandler<InternalRequest>> {
    fn from(context: Context) -> Box<self::event::EventHandler<InternalRequest>> {
        match context {
            Context::Linear(context) => Box::new(context),
            Context::Map(context) => Box::new(context),
        }
    }
}

pub mod linear {
    use uuid::Uuid;
    use std::rc::Rc;

    use context;
    use Conditions;
    use context::event::{EventHandler};
    use message::{Message};
    use state::State;
    use TimerEvent;
    use dispatcher::request::{InternalRequest, Request};
    use context::base::BaseContext;

    pub struct LinearContext {
        base: BaseContext,
        state: State
    }

    impl LinearContext {
        pub fn new(uuid: Uuid, conditions: Conditions) -> LinearContext {
            LinearContext {
                base: BaseContext::new(uuid, conditions),
                state: State::new()
            }
        }

        pub fn on_event(&mut self, event: InternalRequest) {
            if let Request::Timer(event) = event {
                self.on_timer(&event);
            }
        }

        pub fn on_timer(&mut self, event: &TimerEvent) {
            self.base.on_timer(event, &mut self.state)
        }

        pub fn on_message(&mut self, event: Rc<Message>) {
            self.base.on_message(event, &mut self.state);
        }

        pub fn is_open(&self) -> bool {
            self.state.is_open()
        }

        pub fn patterns(&self) -> &[String] {
            &self.base.conditions().patterns
        }
    }

    impl From<BaseContext> for LinearContext {
        fn from(context: BaseContext) -> LinearContext {
            LinearContext {
                base: context,
                state: State::new()
            }
        }
    }

    impl EventHandler<InternalRequest> for LinearContext {
        fn handlers(&self) -> &[String] {
            self.patterns()
        }
        fn handle_event(&mut self, event: InternalRequest) {
            self.on_event(event);
        }
    }

    impl From<LinearContext> for Box<context::event::EventHandler<InternalRequest>> {
        fn from(context: LinearContext) -> Box<context::event::EventHandler<InternalRequest>> {
            Box::new(context)
        }
    }
}


#[cfg(test)]
mod test {
    use uuid::Uuid;
    use std::rc::Rc;

    use message;
    use TimerEvent;
    use context::LinearContext;
    use conditions::Builder;

    #[test]
    fn test_given_close_condition_with_timeout_when_the_timeout_expires_then_the_condition_is_met() {
        let timeout = 100;
        let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let patterns = vec![
            msg_id.clone(),
        ];
        let mut context = LinearContext::new(Uuid::new_v4(), Builder::new(timeout).patterns(patterns).build());
        let msg1 = message::Builder::new(&msg_id).build();
        let event = Rc::new(msg1);
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
        let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let patterns = vec![
            msg_id.clone(),
        ];
        let mut context = LinearContext::new(Uuid::new_v4(), Builder::new(timeout).max_size(max_size).patterns(patterns).build());
        let msg1 = message::Builder::new(&msg_id).build();
        let event = Rc::new(msg1);
        context.on_message(event.clone());
        assert_true!(context.is_open());
        context.on_message(event.clone());
        assert_true!(context.is_open());
        context.on_message(event.clone());
        assert_false!(context.is_open());
    }

    #[test]
    fn test_given_close_condition_with_renew_timeout_when_the_timeout_expires_without_renewing_messages_then_the_condition_is_met() {
        let timeout = 100;
        let renew_timeout = 10;
        let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let patterns = vec![
            msg_id.clone(),
        ];
        let mut context = LinearContext::new(Uuid::new_v4(), Builder::new(timeout).renew_timeout(renew_timeout).patterns(patterns).build());
        let msg1 = message::Builder::new(&msg_id).build();
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
        let msg_id = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
        let patterns = vec![
            msg_id.clone(),
        ];
        let mut context = LinearContext::new(Uuid::new_v4(), Builder::new(timeout).renew_timeout(renew_timeout).patterns(patterns).build());
        let msg1 = message::Builder::new(&msg_id).build();
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
