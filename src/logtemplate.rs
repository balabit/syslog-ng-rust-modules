use std::sync::Arc;

use syslog_ng_common::{self, GlobalConfig};

use logevent::LogEvent;
use correlation::{Template, TemplateFactory, CompileError};

unsafe impl Send for LogTemplate {}

pub struct LogTemplate(syslog_ng_common::LogTemplate);

impl Template for LogTemplate {
    type Event = LogEvent;
    fn format_with_context(&self, _: &[Arc<Self::Event>], _: &str, _: &mut String) {

    }
}

// pub struct LogTemplateFactory(Rc<GlobalConfig>);
#[derive(Default)]
pub struct LogTemplateFactory;

impl TemplateFactory<LogEvent> for LogTemplateFactory {
    type Template = LogTemplate;
    fn compile(&self, value: &str) -> Result<Self::Template, CompileError> {
        Err(CompileError("wtf".to_owned()))
    }
}
