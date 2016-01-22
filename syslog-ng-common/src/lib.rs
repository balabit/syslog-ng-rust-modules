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
extern crate syslog_ng_sys;

#[macro_use]
mod proxies;
mod logger;
mod messages;
mod formatter;
mod logmsg;
mod cfg;
pub mod sys;
mod logparser;

pub use syslog_ng_sys::{c_int, c_char, ssize_t};
pub use logparser::LogParser;
pub use logmsg::LogMessage;
pub use formatter::MessageFormatter;
pub use logger::init_logger;
pub use cfg::GlobalConfig;
pub use proxies::parser::{OptionError, Parser, ParserBuilder, ParserProxy};
