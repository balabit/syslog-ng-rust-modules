use syslog_ng_sys;
use LogMessage;

use syslog_ng_sys::logpipe::__log_pipe_forward_msg;
use syslog_ng_sys::LogPathOptions;

pub trait Pipe {
    fn forward(&mut self, msg: LogMessage);
}

pub struct LogPipe(pub *mut syslog_ng_sys::LogPipe);

impl Pipe for LogPipe {
    fn forward(&mut self, msg: LogMessage) {
        let mut path_options = LogPathOptions::default();
        path_options.ack_needed = 0;
        unsafe {
            __log_pipe_forward_msg(self.0 as *mut syslog_ng_sys::LogPipe, msg.into_raw(), &path_options);
        }
    }
}
