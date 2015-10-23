pub use syslog_ng_sys::{
    c_int,
    c_char,
    ssize_t
};

pub use syslog_ng_sys::{
    from_c_str_to_owned_string,
    from_c_str_to_borrowed_str
};

pub use syslog_ng_sys::logmsg::LogMessage;
pub use syslog_ng_sys::logparser::LogParser;
