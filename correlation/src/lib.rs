// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]
#![cfg_attr(feature="nightly", deny(warnings))]

#[macro_use]
extern crate maplit;
extern crate env_logger;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate rustc_serialize;
#[macro_use]
extern crate log;

#[macro_use]
mod macros;

pub use action::Alert;
pub use conditions::{Conditions, ConditionsBuilder};
pub use config::action::ActionType;
pub use dispatcher::{Response, ResponseHandle};
pub use dispatcher::request::Request;
pub use message::{Message, MessageBuilder};
pub use context::ContextMap;
pub use reactor::{EventHandler, SharedData};

pub mod config;
pub mod correlator;
pub mod test_utils;
mod conditions;
mod action;
mod message;
mod context;
mod dispatcher;
mod reactor;
mod state;
mod timer;
mod duration;

pub trait Event {
    fn get(&self, key: &str) -> Option<&str>;
    fn ids(&self) -> &[String];
}

pub trait Template<E: Event> {
    fn format(&self, msg: &[E]) -> Result<&str, String>;
}
