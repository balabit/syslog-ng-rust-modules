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

pub trait Event: Send + Sync + Clone {
    fn get(&self, key: &str) -> Option<&str>;
    fn uuid(&self) -> &str;
    fn ids(&self) -> EventIds;
    fn new(uuid: &str, message: &str) -> Self;
    fn set_name(&mut self, name: Option<&str>);
    fn name(&self) -> Option<&str>;
    fn set(&mut self, key: &str, value: &str);
    fn set_message(&mut self, message: &str);
    fn message(&self) -> &str;
}

pub struct EventIds<'a> {
    pub uuid: &'a str,
    pub name: Option<&'a str>
}

impl<'a> IntoIterator for EventIds<'a> {
    type Item = &'a str;
    type IntoIter = EventIdsIterator<'a>;

    fn into_iter(self) -> EventIdsIterator<'a> {
        EventIdsIterator {
            ids: self,
            state: 0
        }
    }
}

pub struct EventIdsIterator<'ids> {
    ids: EventIds<'ids>,
    state: u8,
}

impl<'a> Iterator for EventIdsIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                self.state += 1;
                Some(self.ids.uuid)
            }
            1 => {
                self.state += 1;
                self.ids.name
            }
            _ => None,
        }
    }
}

pub trait TemplateFactory<E> where E: Event {
    type Template: Template<Event=E>;
    fn compile(&self, value: &str) -> Result<Self::Template, CompileError>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct CompileError(pub String);

use std::fmt::{Display, Formatter, Error as FmtError};

impl Display for CompileError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FmtError> {
        formatter.write_str(&self.0)
    }
}

impl ::std::error::Error for CompileError {
    fn description(&self) -> &str {
        "Failed to compile template expression"
    }
    fn cause(&self) -> Option<&::std::error::Error> { None }
}

use std::sync::Arc;

pub trait Template: Send {
    type Event: Event;
    fn format_with_context(&self, messages: &[Arc<Self::Event>], context_id: &str, buffer: &mut String);
}
