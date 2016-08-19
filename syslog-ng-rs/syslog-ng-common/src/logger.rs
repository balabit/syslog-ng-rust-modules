// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use log::{LogRecord, LogMetadata, Log, LogLevelFilter};
use messages::{InternalMessageSender, Msg};

use log;

/// Initializes the logging subsystem to use syslog-ng's internal logs as backend.
pub fn init_logger() {
    let _ = log::set_logger(|max_log_level| {
        max_log_level.set(InternalLogger::level());
        Box::new(InternalLogger)
    });
}

/// Sends logs into syslog-ng's internal log stream.
pub struct InternalLogger;

impl InternalLogger {
    /// Creates a log filter based on the current log level of syslog-ng.
    pub fn level() -> LogLevelFilter {
        InternalMessageSender::level()
    }
}

impl Log for InternalLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= InternalMessageSender::level()
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let message = format!("{}", record.args());
            let level = Msg::from(record.level());
            InternalMessageSender::create_and_send(level, message);
        }
    }
}
