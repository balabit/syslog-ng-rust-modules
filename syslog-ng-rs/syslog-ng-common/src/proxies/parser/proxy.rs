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

pub use proxies::parser::{Error, Parser, ParserBuilder};

#[repr(C)]
pub struct ParserProxy<B>
    where B: ParserBuilder<LogParser>
{
    parser: Option<B::Parser>,
    builder: Option<B>,
}

impl<B> ParserProxy<B> where B: ParserBuilder<LogParser>
{
    pub fn new(cfg: GlobalConfig) -> ParserProxy<B> {
        ParserProxy {
            parser: None,
            builder: Some(B::new(cfg)),
        }
    }

    pub fn with_builder_and_parser(builder: Option<B>,
                                   parser: Option<B::Parser>)
                                   -> ParserProxy<B> {
        ParserProxy {
            parser: parser,
            builder: builder,
        }
    }

    fn build_parser(&mut self, builder: B) -> bool {
        match builder.build() {
            Ok(mut parser) => {
                let init_result = parser.init();
                self.parser = Some(parser);
                init_result
            }
            Err(error) => {
                error!("Error: {:?}", error);
                false
            }
        }
    }

    pub fn init(&mut self) -> bool {
        if let Some(builder) = self.builder.take()  {
            return self.build_parser(builder);
        }

        if let Some(ref mut parser) = self.parser {
            parser.init()
        } else {
            false
        }
    }

    pub fn deinit(&mut self) -> bool {
        if let Some(ref mut parser) = self.parser {
            parser.deinit()
        } else {
            false
        }
    }

    pub fn set_option(&mut self, name: String, value: String) -> bool {
        let builder = self.builder.as_mut().expect("Failed to get builder on a ParserProxy");

        match builder.option(name, value) {
            Ok(()) => true,
            Err(error) => {
                error!("{}", error);
                false
            }
        }
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
        ParserProxy {parser: None, builder: self.builder.clone()}
    }
}
