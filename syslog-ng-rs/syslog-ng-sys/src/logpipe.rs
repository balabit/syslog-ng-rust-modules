use LogMessage;
use types::c_int;

#[repr(C)]
pub struct LogPathOptions {
    pub ack_needed: c_int,
    pub flow_control_requested: c_int,
    pub matched: *mut c_int
}

impl Default for LogPathOptions {
    fn default() -> LogPathOptions {
        LogPathOptions {ack_needed: 0, flow_control_requested: 0, matched: ::std::ptr::null_mut()}
    }
}

pub enum LogPipe {}

#[link(name = "syslog-ng")]
extern "C" {
    pub fn __log_pipe_forward_msg(slf: *mut LogPipe, msg: *mut LogMessage, path_options: *const LogPathOptions);
}
