// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! This module reexports some types from the `syslog_ng_sys` crate.
//!
//! The users of `syslog-ng-common` shouldn't use `syslog-ng-sys` directly, instead use the high
//! level wrappers provided by `syslog-ng-common` (they don't have to define `syslog-ng-sys` as a
//! dependency, since it's pulled in by syslog-ng-common). However, some functions are useful
//! for the users of this crate, so they are reexported under this module.
pub use syslog_ng_sys::LogMessage;
pub use syslog_ng_sys::LogParser;
pub use syslog_ng_sys::GlobalConfig;

pub mod logmsg {
    pub use syslog_ng_sys::logmsg::log_msg_registry_init;
    pub use syslog_ng_sys::logmsg::log_msg_registry_deinit;
    pub use syslog_ng_sys::logmsg::log_tags_global_init;
}

pub mod logtemplate {
    pub use syslog_ng_sys::logtemplate::log_template_global_init;
    pub use syslog_ng_sys::logtemplate::log_template_global_deinit;
}
