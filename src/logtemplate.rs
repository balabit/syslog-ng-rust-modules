use std::sync::Arc;
use std::fmt::Write;
use std::cell::RefCell;

use syslog_ng_common::{self, GlobalConfig, LogTimeZone};

use logevent::LogEvent;
use correlation::{Template, TemplateFactory, CompileError};

unsafe impl Send for LogTemplate {}

pub struct LogTemplate(RefCell<syslog_ng_common::LogTemplate>);

impl Template for LogTemplate {
    type Event = LogEvent;
    fn format_with_context(&self, messages: &[Arc<Self::Event>], context_id: &str, buffer: &mut String) {
        let messages: Vec<syslog_ng_common::LogMessage> = messages.iter().map(|event| event.0.clone()).collect();
        let mut template = self.0.borrow_mut();
        let formatted_str = template.format_with_context(&messages, None, LogTimeZone::Send, 0, Some(context_id));
        let _ = buffer.write_str(formatted_str);
    }
}

pub struct LogTemplateFactory(GlobalConfig);

impl TemplateFactory<LogEvent> for LogTemplateFactory {
    type Template = LogTemplate;
    fn compile(&self, value: &str) -> Result<Self::Template, CompileError> {
        syslog_ng_common::LogTemplate::compile(&self.0, value)
                                      .map(|template| LogTemplate(RefCell::new(template)))
                                      .map_err(|err| CompileError(err.to_string()))
    }
}

impl From<GlobalConfig> for LogTemplateFactory {
    fn from(cfg: GlobalConfig) -> LogTemplateFactory {
        LogTemplateFactory(cfg)
    }
}
