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
