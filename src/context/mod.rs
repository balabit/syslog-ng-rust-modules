// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use dispatcher::request::Request;
use dispatcher::response::ResponseSender;
use config::ContextConfig;

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

pub enum Context {
    Linear(LinearContext),
    Map(MapContext),
}

impl Context {
    pub fn on_event(&mut self, event: Request, responder: &mut ResponseSender) {
        match *self {
            Context::Linear(ref mut context) => context.on_event(event, responder),
            Context::Map(ref mut context) => context.on_event(event, responder),
        }
    }

    pub fn patterns(&self) -> &[String] {
        match *self {
            Context::Linear(ref context) => context.patterns(),
            Context::Map(ref context) => context.patterns(),
        }
    }
}

impl From<ContextConfig> for Context {
    fn from(config: ContextConfig) -> Context {
        let ContextConfig {name, uuid, conditions, context_id, actions, patterns} = config;

        let base = BaseContextBuilder::new(uuid, conditions);
        let base = base.name(name);
        let base = base.actions(actions);
        let base = base.build();

        if let Some(context_id) = context_id {
            Context::Map(MapContext::new(base, context_id))
        } else {
            Context::Linear(LinearContext::new(base))
        }
    }
}
