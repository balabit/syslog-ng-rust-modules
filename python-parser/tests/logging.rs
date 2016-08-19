extern crate python_parser;
extern crate syslog_ng_common;
extern crate cpython;
extern crate log;

use std::env;
use python_parser::{options, PythonParserBuilder};
use syslog_ng_common::{ParserBuilder, SYSLOG_NG_INITIALIZED, syslog_ng_global_init, GlobalConfig};
use log::{LogRecord, LogLevel, LogMetadata};

use std::sync::Arc;
use std::sync::Mutex;

#[derive(Eq, PartialEq, Debug)]
struct SimplifiedLogRecord {
    level: LogLevel,
    formatted_message: String,
}

impl SimplifiedLogRecord {
    pub fn new<S: Into<String>>(level: LogLevel, msg: S) -> SimplifiedLogRecord {
        SimplifiedLogRecord {
            level: level,
            formatted_message: msg.into(),
        }
    }
}

struct MockLogger {
    pub messages: Arc<Mutex<Vec<SimplifiedLogRecord>>>,
}

impl MockLogger {
    pub fn new() -> MockLogger {
        MockLogger { messages: Arc::new(Mutex::new(Vec::new())) }
    }
}

impl log::Log for MockLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Trace
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let mut lock = self.messages.lock().unwrap();
            let record = SimplifiedLogRecord::new(record.level(), format!("{}", record.args()));
            lock.push(record);
        }
    }
}

fn init_logging(logger: MockLogger) -> Result<(), log::SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log::LogLevelFilter::Trace);
        Box::new(logger)
    })
}

fn logging_callbacks_can_be_used_from_init_method(messages: Arc<Mutex<Vec<SimplifiedLogRecord>>>) {
    let expected = [SimplifiedLogRecord::new(LogLevel::Info, "INFO"),
                    SimplifiedLogRecord::new(LogLevel::Warn, "WARNING"),
                    SimplifiedLogRecord::new(LogLevel::Trace, "TRACE"),
                    SimplifiedLogRecord::new(LogLevel::Error, "ERROR"),
                    SimplifiedLogRecord::new(LogLevel::Debug, "DEBUG")];
    let cfg = GlobalConfig::new(0x0308);
    let mut builder = PythonParserBuilder::new(cfg);
    builder.option(options::MODULE.to_owned(), "_test_module".to_owned()).ok().unwrap();
    builder.option(options::CLASS.to_owned(),
                "LoggingIsUsedInInitMethod".to_owned())
        .ok()
        .unwrap();
    let _ = builder.build();
    let lock = messages.lock().unwrap();
    for i in &expected {
        assert!((*lock).contains(i),
                "This item wasn't found in the expected messages: {:?}",
                i);
    }
}

fn logging_callbacks_are_not_overriden_if_they_are_already_defined(messages: Arc<Mutex<Vec<SimplifiedLogRecord>>>) {
    let expected = [SimplifiedLogRecord::new(LogLevel::Warn,
                                             "Already implemented info() function, omitting callback definition."),
                    SimplifiedLogRecord::new(LogLevel::Warn,
                                             "Already implemented warning() function, omitting callback definition."),
                    SimplifiedLogRecord::new(LogLevel::Warn,
                                             "Already implemented trace() function, omitting callback definition."),
                    SimplifiedLogRecord::new(LogLevel::Warn,
                                             "Already implemented error() function, omitting callback definition."),
                    SimplifiedLogRecord::new(LogLevel::Warn,
                                             "Already implemented debug() function, omitting callback definition.")];
    let cfg = GlobalConfig::new(0x0308);
    let mut builder = PythonParserBuilder::new(cfg);
    builder.option(options::MODULE.to_owned(),
                "_test_module.test_logging".to_owned())
        .ok()
        .unwrap();
    builder.option(options::CLASS.to_owned(),
                "LoggingCallbacksAreNotOverriden".to_owned())
        .ok()
        .unwrap();
    let _ = builder.build();
    let lock = messages.lock().unwrap();
    for i in &expected {
        assert!((*lock).contains(i),
                "This item wasn't found in the expected messages: {:?}",
                i);
    }
}

fn set_up() -> Arc<Mutex<Vec<SimplifiedLogRecord>>> {
    let logger = MockLogger::new();
    let messages = logger.messages.clone();
    let _ = init_logging(logger);
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe {
            syslog_ng_global_init();
        }
    });
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    messages
}

#[test]
fn test_run_tests() {
    let messages = set_up();
    logging_callbacks_can_be_used_from_init_method(messages.clone());
    logging_callbacks_are_not_overriden_if_they_are_already_defined(messages);
}
