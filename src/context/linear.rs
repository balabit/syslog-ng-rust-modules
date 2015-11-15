use uuid::Uuid;
use std::rc::Rc;

use conditions::Conditions;
use message::Message;
use state::State;
use timer::TimerEvent;
use dispatcher::request::{InternalRequest, Request};
use context::base::{BaseContext, BaseContextBuilder};

pub struct LinearContext {
    base: BaseContext,
    state: State,
}

impl LinearContext {
    pub fn new(uuid: Uuid, conditions: Conditions) -> LinearContext {
        LinearContext {
            base: BaseContextBuilder::new(uuid, conditions).build(),
            state: State::new(),
        }
    }

    pub fn on_event(&mut self, event: InternalRequest) {
        trace!("LinearContext: received event");
        match event {
            Request::Timer(event) => {
                self.on_timer(&event)
            }
            Request::Message(message) => {
                self.on_message(message)
            }
            _ => {}
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

    pub fn uuid(&self) -> &Uuid {
        self.base.uuid()
    }
}

impl From<BaseContext> for LinearContext {
    fn from(context: BaseContext) -> LinearContext {
        LinearContext {
            base: context,
            state: State::new(),
        }
    }
}
