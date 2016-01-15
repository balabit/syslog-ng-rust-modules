#[macro_use]
extern crate log;

#[macro_use]
extern crate syslog_ng_sys;

#[macro_use]
pub mod proxies;
pub mod logger;
pub mod messages;
pub mod formatter;
pub mod logmsg;
pub mod cfg;
pub mod sys;

pub use syslog_ng_sys::{c_int, c_char, ssize_t};

pub use logmsg::LogMessage;
pub use syslog_ng_sys::logparser::LogParser;
