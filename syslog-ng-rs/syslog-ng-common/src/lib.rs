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
extern crate glib;
extern crate glib_sys;
extern crate libc;

use std::sync::{Once, ONCE_INIT};
use libc::abort;

#[macro_use]
mod proxies;
mod logger;
mod messages;
mod formatter;
mod logmsg;
mod cfg;
pub mod sys;
mod logparser;
mod logpipe;
pub mod mock;
pub mod logtemplate;
mod plugin;

pub use syslog_ng_sys::{c_int, c_char, ssize_t};
pub use logparser::LogParser;
pub use logmsg::LogMessage;
pub use formatter::MessageFormatter;
pub use logger::init_logger;
pub use cfg::GlobalConfig;
pub use proxies::parser::{OptionError, Parser, ParserBuilder, ParserProxy};
pub use logpipe::{LogPipe, Pipe};
pub use logtemplate::{LogTemplate, LogTemplateOptions, LogTimeZone};
pub use plugin::Plugin;

#[allow(dead_code)]
pub static SYSLOG_NG_INITIALIZED: Once = ONCE_INIT;

pub unsafe fn syslog_ng_global_init() {
    use syslog_ng_sys::resolved_configurable_paths as c_paths;

    c_paths::resolved_configurable_paths_init(&mut c_paths::resolvedConfigurablePaths);
    sys::logmsg::log_msg_registry_init();
    sys::logmsg::log_tags_global_init();
    sys::logtemplate::log_template_global_init();
}

pub fn commit_suicide() -> ! {
    unsafe {
        abort();
    };
}
