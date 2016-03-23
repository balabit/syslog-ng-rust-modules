// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate syslog_ng_common;
#[macro_use]
extern crate log;

use std::marker::PhantomData;

pub struct DummyParser<P: Pipe>(PhantomData<P>);

impl<P: Pipe> Clone for DummyParser<P> {
    fn clone(&self) -> DummyParser<P> {
        DummyParser(self.0.clone())
    }
}

pub struct DummyParserBuilder<P: Pipe>(PhantomData<P>);

use syslog_ng_common::LogMessage;
use syslog_ng_common::{Parser, ParserBuilder, OptionError, Pipe};

impl<P: Pipe> ParserBuilder<P> for DummyParserBuilder<P> {
    type Parser = DummyParser<P>;
    fn new() -> Self {
        DummyParserBuilder(PhantomData)
    }
    fn option(&mut self, name: String, value: String) {
        debug!("Setting option: {}={}", name, value);
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        debug!("Building Rust parser");
        Ok(DummyParser(PhantomData))
    }
}

impl<P: Pipe> Parser<P> for DummyParser<P> {
    fn parse(&mut self, _: &mut P, message: &mut LogMessage, input: &str) -> bool {
        debug!("Processing input in Rust Parser: {}", input);
        message.insert("input", input);
        true
    }
}

// this verifies that the macro can be expanded
parser_plugin!(DummyParserBuilder<LogParser>);

use syslog_ng_common::sys::logmsg::log_msg_registry_init;

struct DummyPipe;

impl Pipe for DummyPipe {
    fn forward(&mut self, _: LogMessage) {}
}

#[test]
fn test_given_parser_implementation_when_it_receives_a_message_then_it_adds_a_specific_key_value_pair_to_it
    () {
    unsafe {
        // we may initialize it multiple times -> we leak some memory.
        // we may deinit it after each tests, but it's racy: the tests
        // are running concurrently. That's why I didn't write a guard
        // around it.
        log_msg_registry_init();
    };
    let builder = DummyParserBuilder::<DummyPipe>::new();
    let mut parser = builder.build().ok().expect("Failed to build DummyParser");
    let mut msg = LogMessage::new();
    let input = "The quick brown ...";
    let mut pipe = DummyPipe;
    let result = parser.parse(&mut pipe, &mut msg, input);
    assert!(result);
    assert_eq!(msg.get("input"), input);
}
