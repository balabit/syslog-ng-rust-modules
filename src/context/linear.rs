use uuid::Uuid;
use std::sync::Arc;

use conditions::Conditions;
use message::Message;
use state::State;
use timer::TimerEvent;
use dispatcher::request::Request;
use dispatcher::response::ResponseSender;
use context::base::{BaseContext, BaseContextBuilder};

pub struct LinearContext {
    base: BaseContext,
    state: State,
}

impl LinearContext {
    #[allow(dead_code)]
    pub fn new(uuid: Uuid, conditions: Conditions) -> LinearContext {
        LinearContext {
            base: BaseContextBuilder::new(uuid, conditions).build(),
            state: State::new(),
        }
    }

    pub fn on_event(&mut self, event: Request, responder: &mut ResponseSender) {
        trace!("LinearContext: received event");
        match event {
            Request::Timer(event) => self.on_timer(&event, responder),
            Request::Message(message) => self.on_message(message, responder),
            _ => {}
        }
    }

    pub fn on_timer(&mut self, event: &TimerEvent, responder: &mut ResponseSender) {
        self.state.on_timer(event, &self.base, responder);
    }

    pub fn on_message(&mut self, event: Arc<Message>, responder: &mut ResponseSender) {
        self.state.on_message(event, &self.base, responder);
    }

    #[allow(dead_code)]
    pub fn is_open(&self) -> bool {
        self.state.is_open()
    }

    pub fn patterns(&self) -> &[String] {
        &self.base.conditions().patterns
    }

    #[allow(dead_code)]
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
