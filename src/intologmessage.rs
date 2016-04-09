use logevent::LogEvent;
use correlation::Message;
use syslog_ng_common::LogMessage;

use std::borrow::Borrow;

pub trait IntoLogMessage {
    fn into_logmessage(self) -> LogMessage;
}

impl IntoLogMessage for LogEvent {
    fn into_logmessage(self) -> LogMessage {
        self.0
    }
}

impl IntoLogMessage for Message {
    fn into_logmessage(self) -> LogMessage {
        let mut logmsg = LogMessage::new();
        for (k, v) in self.values.iter() {
            logmsg.insert(k.borrow(), v.borrow());
        }
        logmsg.insert("MESSAGE", &self.message);
        logmsg
    }
}
