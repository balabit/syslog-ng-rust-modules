// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate log;
#[macro_use]
extern crate syslog_ng_common;
extern crate actiondb;

use std::borrow::Borrow;

use actiondb::matcher::{Matcher, PatternLoader, MatcherSuite};
use syslog_ng_common::{Parser, ParserBuilder, OptionError, LogMessage, MessageFormatter, Pipe};

mod msgfilller;
mod keys;
mod options;

use self::msgfilller::MessageFiller;

#[derive(Clone)]
pub struct ActiondbParserBuilder<MS> where MS: MatcherSuite, MS::Matcher: Clone {
    matcher: Option<MS::Matcher>,
    formatter: MessageFormatter
}

impl<MS> ActiondbParserBuilder<MS> where MS: MatcherSuite, MS::Matcher: Clone {
    pub fn set_pattern_file(&mut self, path: &str) {
        match PatternLoader::from_file::<MS::MatcherFactory>(path) {
            Ok(matcher) => self.matcher = Some(matcher),
            Err(err) => {
                error!("ActiondbParser: failed to set 'pattern_file': {}", err);
            }
        }
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.formatter.set_prefix(prefix);
    }
}

impl<MS, P> ParserBuilder<P> for ActiondbParserBuilder<MS> where P: Pipe, MS: MatcherSuite + Clone, MS::Matcher: Clone {
    type Parser = ActiondbParser<MS::Matcher>;
    fn new() -> Self {
        ActiondbParserBuilder {
            matcher: None,
            formatter: MessageFormatter::new(),
        }
    }
    fn option(&mut self, name: String, value: String) {
        trace!("ActiondbParserBuilder: set_option(name={}, value={})",
               &name,
               &value);

        match name.borrow() {
            options::PATTERN_FILE => {
                self.set_pattern_file(&value);
            }
            options::PREFIX => {
                self.set_prefix(value);
            }
            _ => {
                debug!("ActiondbParserBuilder: unsupported option: {:?}", &name) ;
            }
        };

    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        let ActiondbParserBuilder {matcher, formatter} = self;
        debug!("ActiondbParser: building");
        let matcher =
            try!(matcher.ok_or(OptionError::missing_required_option(options::PATTERN_FILE)));
        Ok(ActiondbParser {
            matcher: matcher,
            formatter: formatter,
        })
    }
}

pub struct ActiondbParser<M> where M: Matcher + Clone {
    pub matcher: M,
    pub formatter: MessageFormatter,
}

impl<M, P> Parser<P> for ActiondbParser<M> where P: Pipe, M: Matcher + Clone {
    fn parse(&mut self, _: &mut P, msg: &mut LogMessage, input: &str) -> bool {
        if let Some(result) = self.matcher.parse(input) {
            MessageFiller::fill_logmsg(&mut self.formatter, msg, &result);
            true
        } else {
            false
        }
    }
}

impl<M> Clone for ActiondbParser<M> where M: Matcher + Clone {
    fn clone(&self) -> ActiondbParser<M> {
        ActiondbParser {
            matcher: self.matcher.clone(),
            formatter: self.formatter.clone(),
        }
    }
}


// You can change the matcher implementation by uncommening these lines
// and commenting out the last lines.

//pub use actiondb::matcher::trie::TrieMatcherSuite;
//parser_plugin!(ActiondbParserBuilder<TrieMatcherSuite>);

// Note, that it sould be publicly reexported
pub use actiondb::matcher::suffix_array::SuffixArrayMatcherSuite;
parser_plugin!(ActiondbParserBuilder<SuffixArrayMatcherSuite>);
