use syslog_ng_sys::{self, c_char};
use syslog_ng_sys::logtemplate as sys;
use glib::Error;
use glib_sys;
use LogMessage;

use GlobalConfig;

use std::ffi::{CString, CStr};

#[cfg(test)]
mod tests;

pub struct LogTemplate {
    pub wrapped: *mut sys::LogTemplate,
    buffer: *mut glib_sys::GString,
}

pub struct LogTemplateOptions(pub *mut sys::LogTemplateOptions);

pub enum LogTimeZone {
    Local = 0,
    Send = 1,
}

impl LogTemplate {
    fn new(cfg: &GlobalConfig) -> LogTemplate {
        let raw_cfg = cfg.raw_ptr();
        LogTemplate {
            wrapped: unsafe { sys::log_template_new(raw_cfg, ::std::ptr::null()) },
            buffer: unsafe { glib_sys::g_string_sized_new(128) },
        }
    }
    pub fn compile(cfg: &GlobalConfig, content: &str) -> Result<LogTemplate, Error> {
        let template = LogTemplate::new(cfg);

        let content = CString::new(content).unwrap();
        let mut error = ::std::ptr::null_mut();
        let result = unsafe { sys::log_template_compile(template.wrapped, content.as_ptr(), &mut error) };
        if result != 0 {
            Ok(template)
        } else {
            Err(Error::wrap(error))
        }
    }

    pub fn format(&mut self, msg: &LogMessage, options: Option<&LogTemplateOptions>, tz: LogTimeZone, seq_num: i32, context_id: Option<&str>) -> &str {
        let options: *const sys::LogTemplateOptions = options.map_or(::std::ptr::null(), |options| options.0);
        let result = unsafe {
            let context_id: *const c_char = context_id.map_or(::std::ptr::null(), |id| {
                let cstring = CString::new(id).unwrap();
                cstring.into_raw()
            });

            sys::log_template_format(self.wrapped, msg.0, options, tz as i32, seq_num, context_id, self.buffer);

            if context_id != ::std::ptr::null() {
                let _ = CString::from_raw(context_id as *mut c_char);
            }

            CStr::from_ptr((*self.buffer).str)
        };
        result.to_str().unwrap()
    }

    pub fn format_with_context(&mut self, messages: &[LogMessage], options: Option<&LogTemplateOptions>, tz: LogTimeZone, seq_num: i32, context_id: Option<&str>) -> &str {
        let options: *const sys::LogTemplateOptions = options.map_or(::std::ptr::null(), |options| options.0);
        let messages = messages.iter().map(|msg| msg.0 as *const syslog_ng_sys::LogMessage).collect::<Vec<*const syslog_ng_sys::LogMessage>>();
        let result = unsafe {
            let context_id: *const c_char = context_id.map_or(::std::ptr::null(), |id| {
                let cstring = CString::new(id).unwrap();
                cstring.into_raw()
            });

            sys::log_template_format_with_context(self.wrapped, messages.as_ptr(), messages.len() as i32, options, tz as i32, seq_num, context_id, self.buffer);

            if context_id != ::std::ptr::null() {
                let _ = CString::from_raw(context_id as *mut c_char);
            }

            CStr::from_ptr((*self.buffer).str)
        };
        result.to_str().unwrap()
    }
}

impl Drop for LogTemplate {
    fn drop(&mut self) {
        unsafe {
            sys::log_template_unref(self.wrapped);
            glib_sys::g_string_free(self.buffer, 1 as glib_sys::gboolean);
        };
    }
}
