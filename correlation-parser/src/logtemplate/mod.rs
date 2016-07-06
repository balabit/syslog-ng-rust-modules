use std::io::Write;
use std::cell::RefCell;

use syslog_ng_common::{self, GlobalConfig, LogTimeZone};

use logevent::LogEvent;
use correlation::{Template, TemplateFactory, CompileError};

unsafe impl Send for LogTemplate {}

pub struct LogTemplate(RefCell<syslog_ng_common::LogTemplate>);

impl Template for LogTemplate {
    type Event = LogEvent;
    fn format_with_context(&self, messages: &[Self::Event], context_id: &str, buffer: &mut Write) {
        let messages: Vec<syslog_ng_common::LogMessage> = messages.iter().map(|event| event.0.clone()).collect();
        let mut template = self.0.borrow_mut();
        let formatted_bytes = template.format_with_context(&messages, None, LogTimeZone::Local, 0, context_id);
        let _ = buffer.write(formatted_bytes);
    }
}

pub struct LogTemplateFactory(GlobalConfig);

impl TemplateFactory<LogEvent> for LogTemplateFactory {
    type Template = LogTemplate;
    fn compile(&self, value: &[u8]) -> Result<Self::Template, CompileError> {
        syslog_ng_common::LogTemplate::compile(&self.0, value)
                                      .map(|template| LogTemplate(RefCell::new(template)))
                                      .map_err(|err| { CompileError(err.into_vec()) })
    }
}

impl From<GlobalConfig> for LogTemplateFactory {
    fn from(cfg: GlobalConfig) -> LogTemplateFactory {
        LogTemplateFactory(cfg)
    }
}
