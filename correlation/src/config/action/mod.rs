// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use action::Action;
use state::State;
use context::BaseContext;
use dispatcher::response::ResponseSender;
use self::message::MessageAction;
use Event;
use Template;

pub mod message;
mod deser;

pub enum ActionType<T> {
    Message(MessageAction<T>),
}

impl<E, T> Action<E, T> for ActionType<T> where E: Event, T: Template<Event=E> {
    fn on_opened(&self, state: &State<E>, context: &BaseContext<E, T>, responder: &mut ResponseSender<E>) {
        match *self {
            ActionType::Message(ref action) => action.on_opened(state, context, responder),
        }
    }
    fn on_closed(&self, state: &State<E>, context: &BaseContext<E, T>, responder: &mut ResponseSender<E>) {
        match *self {
            ActionType::Message(ref action) => action.on_closed(state, context, responder),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecCondition {
    pub on_opened: bool,
    pub on_closed: bool,
}

impl ExecCondition {
    pub fn new() -> ExecCondition {
        Default::default()
    }
}

impl Default for ExecCondition {
    fn default() -> ExecCondition {
        ExecCondition {
            on_opened: false,
            on_closed: true,
        }
    }
}
