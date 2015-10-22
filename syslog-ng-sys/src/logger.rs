use log::{LogRecord, LogMetadata, Log, LogLevelFilter};
use messages::{InternalMessageSender, Msg};

pub struct InternalLogger;

impl InternalLogger {
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
