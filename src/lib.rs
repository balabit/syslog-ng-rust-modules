#[macro_use]
extern crate maplit;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate rustc_serialize;
extern crate handlebars;
#[macro_use]
mod macros;

pub use correlator::Correlator;
pub use config::action::ActionType;
pub use dispatcher::{Response};
pub use message::Message;

pub mod conditions;
pub mod config;
pub mod action;
pub mod condition;
mod context;
mod correlator;
mod dispatcher;
pub mod message;
mod state;
mod reactor;
mod timer;

pub type MiliSec = u32;
