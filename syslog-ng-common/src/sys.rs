pub use syslog_ng_sys::LogMessage;
pub use syslog_ng_sys::GlobalConfig;

pub mod logmsg {
    pub use syslog_ng_sys::logmsg::log_msg_registry_init;
    pub use syslog_ng_sys::logmsg::log_msg_registry_deinit;
}
