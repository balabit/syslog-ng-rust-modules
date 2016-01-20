use syslog_ng_sys;

pub struct LogParser(*mut syslog_ng_sys::LogParser);

impl LogParser {
    pub fn wrap_raw(raw: *mut syslog_ng_sys::LogParser) -> LogParser {
        LogParser(raw)
    }
}
