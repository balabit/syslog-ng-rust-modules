#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]
#![cfg_attr(feature="nightly", deny(warnings))]

#[macro_use]
extern crate maplit;
extern crate env_logger;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate rustc_serialize;
extern crate handlebars;
#[macro_use]
extern crate log;

#[macro_use]
mod macros;

pub use conditions::{Conditions, ConditionsBuilder};
pub use config::action::ActionType;
pub use dispatcher::Response;
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
