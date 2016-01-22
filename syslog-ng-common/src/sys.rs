// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

pub use syslog_ng_sys::LogMessage;
pub use syslog_ng_sys::LogParser;
pub use syslog_ng_sys::GlobalConfig;

pub mod logmsg {
    pub use syslog_ng_sys::logmsg::log_msg_registry_init;
    pub use syslog_ng_sys::logmsg::log_msg_registry_deinit;
}
