use std::ffi::CString;
use log::{LogLevel, LogLevelFilter};

mod ffi;

pub enum Msg {
    Fatal = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7
}

impl From<LogLevel> for Msg {
    fn from(level: LogLevel) -> Msg {
        match level {
            LogLevel::Error => Msg::Error,
            LogLevel::Warn => Msg::Warning,
            LogLevel::Info => Msg::Info,
            LogLevel::Debug => Msg::Debug,
            LogLevel::Trace => Msg::Debug
        }
    }
}

pub struct InternalMessageSender;

impl InternalMessageSender {

    pub fn create_and_send(severity: Msg, message: String) {
        unsafe {
            if ffi::debug_flag != 0 {
                let msg = CString::new(message).unwrap();
                let prio = severity as i32;
                let msg_event = ffi::msg_event_create_from_desc(prio, msg.as_ptr());
                ffi::msg_event_suppress_recursions_and_send(msg_event);
            }
        };
    }

    pub fn level() -> LogLevelFilter {
        if ffi::trace_flag != 0 {
            LogLevelFilter::Trace
        } else if ffi::debug_flag != 0 {
            LogLevelFilter::Debug
        } else {
            LogLevelFilter::Info
        }
    }
}
