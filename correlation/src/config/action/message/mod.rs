// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use action::Action;
use context::base::BaseContext;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use message::{Message, MessageBuilder};
use Event;

use std::collections::BTreeMap;
use state::State;
use super::ExecCondition;

pub use self::builder::MessageActionBuilder;

mod deser;
mod builder;
#[cfg(test)]
mod test;

pub const CONTEXT_UUID: &'static str = "context_uuid";
pub const CONTEXT_NAME: &'static str = "context_name";
pub const CONTEXT_LEN: &'static str = "context_len";
pub const MESSAGES: &'static str = "messages";

pub struct MessageAction {
    uuid: String,
    name: Option<String>,
    message: String,
    values: BTreeMap<String, String>,
    when: ExecCondition,
    inject_mode: InjectMode,
}

impl MessageAction {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn message(&self) -> &String {
        &self.message
    }
    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }
    pub fn inject_mode(&self) -> &InjectMode {
        &self.inject_mode
    }

    fn execute<E: Event>(&self, _state: &State<E>, _context: &BaseContext, responder: &mut ResponseSender) {
        let message = MessageBuilder::new(&self.uuid, self.message.clone())
                                    .name(self.name.clone())
                                    .values(self.values.clone())
                                    .build();
        let response = Alert {
            message: message,
            inject_mode: self.inject_mode.clone(),
        };
        responder.send_response(Response::Alert(response));
    }
}

impl From<MessageAction> for super::ActionType {
    fn from(action: MessageAction) -> super::ActionType {
        super::ActionType::Message(action)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InjectMode {
    Log,
    Forward,
    Loopback,
}

impl Default for InjectMode {
    fn default() -> InjectMode {
        InjectMode::Log
    }
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub message: Message,
    pub inject_mode: InjectMode,
}

impl<E: Event> Action<E> for MessageAction {
    fn on_opened(&self, state: &State<E>, context: &BaseContext, responder: &mut ResponseSender) {
        if self.when.on_opened {
            trace!("MessageAction: on_opened()");
            self.execute(state, context, responder);
        }
    }

    fn on_closed(&self, state: &State<E>, context: &BaseContext, responder: &mut ResponseSender) {
        if self.when.on_closed {
            trace!("MessageAction: on_closed()");
            self.execute(state, context, responder);
        }
    }
}
