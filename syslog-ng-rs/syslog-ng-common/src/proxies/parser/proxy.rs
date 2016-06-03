// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use LogMessage;
use LogParser;
use GlobalConfig;

pub use proxies::parser::{OptionError, Parser, ParserBuilder};

#[repr(C)]
pub struct ParserProxy<B>
    where B: ParserBuilder<LogParser>
{
    pub parser: Option<B::Parser>,
    pub builder: Option<B>,
}

impl<B> ParserProxy<B> where B: ParserBuilder<LogParser>
{
    pub fn new(cfg: GlobalConfig) -> ParserProxy<B> {
        ParserProxy {
            parser: None,
            builder: Some(B::new(cfg)),
        }
    }

    pub fn init(&mut self) -> bool {
        let builder = self.builder.take().expect("Called init when builder was not set");
        match builder.build() {
            Ok(parser) => {
                self.parser = Some(parser);
                true
            }
            Err(error) => {
                error!("Error: {:?}", error);
                false
            }
        }
    }

    pub fn set_option(&mut self, name: String, value: String) {
        let builder = self.builder.as_mut().expect("Failed to get builder on a ParserProxy");
        builder.option(name, value);
    }

    pub fn process(&mut self, parent: &mut LogParser, msg: &mut LogMessage, input: &str) -> bool {
        self.parser
            .as_mut()
            .expect("Called process on a non-existing Rust parser")
            .parse(parent, msg, input)
    }
}

impl<B> Clone for ParserProxy<B> where B: ParserBuilder<LogParser> {
    fn clone(&self) -> ParserProxy<B> {
        // it makes no sense to clone() the builder
        ParserProxy {parser: self.parser.clone(), builder: None}
    }
}
