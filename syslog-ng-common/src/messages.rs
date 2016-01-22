// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::ffi::CString;
use log::{LogLevel, LogLevelFilter};
use syslog_ng_sys::messages;


pub enum Msg {
    _Fatal = 2,
    Error = 3,
    Warning = 4,
    _Notice = 5,
    Info = 6,
    Debug = 7,
}

impl From<LogLevel> for Msg {
    fn from(level: LogLevel) -> Msg {
        match level {
            LogLevel::Error => Msg::Error,
            LogLevel::Warn => Msg::Warning,
            LogLevel::Info => Msg::Info,
            LogLevel::Debug => Msg::Debug,
            LogLevel::Trace => Msg::Debug,
        }
    }
}

pub struct InternalMessageSender;

impl InternalMessageSender {
    pub fn create_and_send(severity: Msg, message: String) {
        unsafe {
            if messages::debug_flag != 0 {
                let msg = CString::new(message).unwrap();
                let prio = severity as i32;
                let msg_event = messages::msg_event_create_from_desc(prio, msg.as_ptr());
                messages::msg_event_suppress_recursions_and_send(msg_event);
            }
        };
    }

    pub fn level() -> LogLevelFilter {
        if messages::trace_flag != 0 {
            LogLevelFilter::Trace
        } else if messages::debug_flag != 0 {
            LogLevelFilter::Debug
        } else {
            LogLevelFilter::Info
        }
    }
}
