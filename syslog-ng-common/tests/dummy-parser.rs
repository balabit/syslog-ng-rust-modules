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

#[derive(Clone)]
pub struct DummyParser;

#[derive(Clone)]
pub struct DummyParserBuilder;

use syslog_ng_common::LogMessage;
use syslog_ng_common::{Parser, ParserBuilder, OptionError};

impl ParserBuilder for DummyParserBuilder {
    type Parser = DummyParser;
    fn new() -> Self {
        DummyParserBuilder
    }
    fn option(&mut self, name: String, value: String) {
        debug!("Setting option: {}={}", name, value);
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        debug!("Building Rust parser");
        Ok(DummyParser)
    }
}

impl Parser for DummyParser {
    fn parse(&mut self, message: &mut LogMessage, input: &str) -> bool {
        debug!("Processing input in Rust Parser: {}", input);
        message.insert("input", input);
        true
    }
}

// this verifies that the macro can be expanded
parser_plugin!(DummyParserBuilder);

use syslog_ng_common::sys::logmsg::log_msg_registry_init;

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
    let builder = DummyParserBuilder::new();
    let mut parser = builder.build().ok().expect("Failed to build DummyParser");
    let mut msg = LogMessage::new();
    let input = "The quick brown ...";
    let result = parser.parse(&mut msg, input);
    assert!(result);
    assert_eq!(msg.get("input"), input);
}
