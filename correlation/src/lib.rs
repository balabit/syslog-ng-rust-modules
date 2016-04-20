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
mod reactor;
mod state;
mod duration;

pub trait Event: Clone {
    fn get(&self, key: &[u8]) -> Option<&[u8]>;
    fn uuid(&self) -> &[u8];
    fn ids(&self) -> EventIds;
    fn new(uuid: &[u8], message: &[u8]) -> Self;
    fn set_name(&mut self, name: Option<&[u8]>);
    fn name(&self) -> Option<&[u8]>;
    fn set(&mut self, key: &[u8], value: &[u8]);
    fn set_message(&mut self, message: &[u8]);
    fn message(&self) -> &[u8];
}

pub struct EventIds<'a> {
    pub uuid: &'a [u8],
    pub name: Option<&'a [u8]>
}

impl<'a> IntoIterator for EventIds<'a> {
    type Item = &'a [u8];
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
    type Item = &'a [u8];
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
    fn compile(&self, value: &[u8]) -> Result<Self::Template, CompileError>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct CompileError(pub Vec<u8>);

use std::fmt::{Display, Formatter, Error as FmtError};

impl Display for CompileError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FmtError> {
        let as_str = String::from_utf8_lossy(&self.0[..]);
        formatter.write_str(&as_str[..])
    }
}

impl ::std::error::Error for CompileError {
    fn description(&self) -> &str {
        "Failed to compile template expression"
    }
    fn cause(&self) -> Option<&::std::error::Error> { None }
}

use std::io::Write;

pub trait Template: Send {
    type Event: Event;
    fn format_with_context(&self, messages: &[Self::Event], context_id: &str, buffer: &mut Write);
}
