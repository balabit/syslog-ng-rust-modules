use log;

use syslog_ng_sys::InternalLogger;

pub mod filter;
pub mod parser;

fn init_logger() {
    let _ = log::set_logger(|max_log_level| {
        max_log_level.set(InternalLogger::level());
        Box::new(InternalLogger)
    });
}
