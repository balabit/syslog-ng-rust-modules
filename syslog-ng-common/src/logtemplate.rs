use syslog_ng_sys::logtemplate as sys;
use glib::Error;
use glib_sys;
use LogMessage;

use GlobalConfig;

use std::ffi::{CString, CStr};

pub struct LogTemplate{
    pub wrapped: *mut sys::LogTemplate,
    buffer: *mut glib_sys::GString
}

pub struct LogTemplateOptions(pub *mut sys::LogTemplateOptions);

pub enum LogTimeZone {
    Local = 0,
    Send = 1,
}

impl LogTemplate {
    pub fn new(cfg: &GlobalConfig) -> LogTemplate {
        LogTemplate {
            wrapped: unsafe { sys::log_template_new(cfg.0, ::std::ptr::null()) },
            buffer: unsafe { glib_sys::g_string_sized_new(128) }
        }
    }
    pub fn compile(content: &str) -> Result<LogTemplate, Error> {
        let cfg = GlobalConfig::new(0x0308);
        let template = LogTemplate::new(&cfg);

        let content = CString::new(content).unwrap();
        let mut error = ::std::ptr::null_mut();
        let result = unsafe { sys::log_template_compile(template.wrapped, content.as_ptr(), &mut error) };
        if result != 0 {
            Ok(template)
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn format(&self, msg: &LogMessage, options: Option<&LogTemplateOptions>, tz: LogTimeZone, seq_num: i32, context_id: Option<&str>) -> &str {
        let options: *const sys::LogTemplateOptions = options.map_or(::std::ptr::null(), |options| options.0);
        let result = unsafe {
            if let Some(context_id) = context_id {
                let context_id = CString::new(context_id).unwrap();
                sys::log_template_format(self.wrapped, msg.0, options, tz as i32, seq_num, context_id.as_ptr(), self.buffer);
            } else {
                sys::log_template_format(self.wrapped, msg.0, options, tz as i32, seq_num, ::std::ptr::null(), self.buffer);
            }
            CStr::from_ptr((*self.buffer).str)
        };
        result.to_str().unwrap()
    }
}

impl Drop for LogTemplate {
    fn drop(&mut self) {
        unsafe { sys::log_template_unref(self.wrapped) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use GlobalConfig;
    use LogMessage;
    use ::sys::logmsg::log_msg_registry_init;
    use ::sys::logtemplate::log_template_global_init;

    #[test]
    fn test_template_can_be_created() {
        let cfg = GlobalConfig::new(0x0308);
        let _ = LogTemplate::new(&cfg);
    }

    #[test]
    fn test_template_can_be_compiled() {
        let _ = LogTemplate::compile("literal").ok().unwrap();
    }

    #[test]
    fn test_invalid_template_cannot_be_compiled() {
        let _ = LogTemplate::compile("${unbalanced").err().unwrap();
    }

    #[test]
    fn test_log_message_can_be_formatted() {
        unsafe {
            log_msg_registry_init();
            log_template_global_init();
        }
        let template = LogTemplate::compile("${kittens}").ok().unwrap();
        let mut msg = LogMessage::new();
        msg.insert("kittens", "2");
        let formatted_msg = template.format(&msg, None, LogTimeZone::Local, 0, None);
        assert_eq!("2", formatted_msg);
    }
}
