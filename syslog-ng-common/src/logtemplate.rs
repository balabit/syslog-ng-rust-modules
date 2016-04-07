use syslog_ng_sys::logtemplate as sys;
use glib::Error;
use LogMessage;

use GlobalConfig;

use std::ffi::CString;

pub struct LogTemplate(pub *mut sys::LogTemplate);
pub struct LogTemplateOptions(pub *mut sys::LogTemplateOptions);

pub enum LogTimeZone {
    Local = 0,
    Send = 1,
}

impl LogTemplate {
    pub fn new(cfg: &GlobalConfig) -> LogTemplate {
        LogTemplate (
            unsafe { sys::log_template_new(cfg.0, ::std::ptr::null()) }
        )
    }
    pub fn compile(content: &str) -> Result<LogTemplate, Error> {
        let cfg = GlobalConfig::new(0x0308);
        let template = LogTemplate::new(&cfg);

        let content = CString::new(content).unwrap();
        let mut error = ::std::ptr::null_mut();
        let result = unsafe { sys::log_template_compile(template.0, content.as_ptr(), &mut error) };
        if result != 0 {
            Ok(template)
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn format(&self, _msg: &LogMessage, _options: Option<&LogTemplateOptions>, _tz: LogTimeZone, _seq_num: i32, _context_id: Option<&str>) -> &str {
        unimplemented!();
    }

   // pub fn log_template_format(slf: *const LogTemplate,
   //                            lm: *const LogMessage,
   //                            opts: *const LogTemplateOptions,
   //                            tz: c_int,
   //                            seq_num: i32,
   //                            context_id: *const c_char,
   //                            result: *mut GString) -> c_void;
}

impl Drop for LogTemplate {
    fn drop(&mut self) {
        unsafe { sys::log_template_unref(self.0) };
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
