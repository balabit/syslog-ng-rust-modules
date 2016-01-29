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
extern crate num;

#[macro_use]
mod macros;

pub use correlator::Correlator;
pub use config::action::ActionType;
pub use dispatcher::Response;
pub use message::Message;

pub mod conditions;
pub mod config;
pub mod action;
pub mod condition;
mod context;
pub mod correlator;
pub mod dispatcher;
pub mod message;
pub mod reactor;
mod state;
pub mod test_utils;
mod timer;

pub type MiliSec = u64;
