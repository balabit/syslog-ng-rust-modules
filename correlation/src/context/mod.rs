// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::VecDeque;
use std::time::Duration;

use config::ContextConfig;
use Event;
use Template;
use Alert;

pub use self::linear::LinearContext;
pub use self::map::MapContext;
pub use self::base::BaseContext;
pub use self::base::BaseContextBuilder;
pub use self::context_map::ContextMap;

pub mod base;
pub mod context_map;
pub mod linear;
pub mod map;
#[cfg(test)]
mod test;

pub enum Context<E, T> where E: Event, T: Template<Event=E> {
    Linear(LinearContext<E, T>),
    Map(MapContext<E, T>),
}

impl<E, T> Context<E, T> where E: Event, T: Template<Event=E> {
    pub fn on_message(&mut self, event: E, responder: &mut VecDeque<Alert<E>>) {
        match *self {
            Context::Linear(ref mut context) => context.on_message(event, responder),
            Context::Map(ref mut context) => context.on_message(event, responder),
        }
    }

    pub fn on_timer(&mut self, event: &Duration, responder: &mut VecDeque<Alert<E>>) {
        match *self {
            Context::Linear(ref mut context) => context.on_timer(event, responder),
            Context::Map(ref mut context) => context.on_timer(event, responder),
        }
    }

    pub fn patterns(&self) -> &[String] {
        match *self {
            Context::Linear(ref context) => context.patterns(),
            Context::Map(ref context) => context.patterns(),
        }
    }
}

impl<E, T> From<ContextConfig<T>> for Context<E, T> where E: Event, T: Template<Event=E> {
    fn from(config: ContextConfig<T>) -> Context<E, T> {
        let ContextConfig {name, uuid, conditions, context_id, actions, patterns} = config;
        let base = BaseContext {uuid: uuid, conditions: conditions, name: name, patterns: patterns, actions: actions};

        if let Some(context_id) = context_id {
            Context::Map(MapContext::new(base, context_id))
        } else {
            Context::Linear(LinearContext::new(base))
        }
    }
}
