use syslog_ng_sys;
use syslog_ng_sys::logtemplate as sys;
use glib_sys;
use glib;
use LogMessage;

use GlobalConfig;

use std::slice::from_raw_parts;
use std::ffi::{CString, NulError};

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

pub enum Error {
    Glib(glib::Error),
    Nul(NulError)
}

impl From<NulError> for Error {
    fn from(error: NulError) -> Error {
        Error::Nul(error)
    }
}

impl Error {
    pub fn into_vec(self) -> Vec<u8> {
        match self {
            Error::Glib(error) => error.to_string().into(),
            Error::Nul(error) => error.into_vec(),
        }
    }
}

impl LogTemplate {
    fn new(cfg: &GlobalConfig) -> LogTemplate {
        let raw_cfg = cfg.raw_ptr();
        LogTemplate {
            wrapped: unsafe { sys::log_template_new(raw_cfg, ::std::ptr::null()) },
            buffer: unsafe { glib_sys::g_string_sized_new(128) },
        }
    }
    pub fn compile(cfg: &GlobalConfig, content: &[u8]) -> Result<LogTemplate, Error> {
        let template = LogTemplate::new(cfg);
        let content = try!(CString::new(content));
        let mut error = ::std::ptr::null_mut();
        let result = unsafe { sys::log_template_compile(template.wrapped, content.as_ptr(), &mut error) };
        if result != 0 {
            Ok(template)
        } else {
            Err(Error::Glib(glib::Error::wrap(error)))
        }
    }

    pub fn format(&mut self, msg: &LogMessage, options: Option<&LogTemplateOptions>, tz: LogTimeZone, seq_num: i32) -> &[u8] {
        let options: *const sys::LogTemplateOptions = options.map_or(::std::ptr::null(), |options| options.0);
        unsafe {
            sys::log_template_format(self.wrapped, msg.0, options, tz as i32, seq_num, ::std::ptr::null(), self.buffer);
            from_raw_parts((*self.buffer).str as *const u8, (*self.buffer).len)
        }
    }

    pub fn format_with_context(&mut self, messages: &[LogMessage], options: Option<&LogTemplateOptions>, tz: LogTimeZone, seq_num: i32, context_id: &str) -> &[u8] {
        let options: *const sys::LogTemplateOptions = options.map_or(::std::ptr::null(), |options| options.0);
        let messages = messages.iter().map(|msg| msg.0 as *const syslog_ng_sys::LogMessage).collect::<Vec<*const syslog_ng_sys::LogMessage>>();
        let context_id = CString::new(context_id).unwrap();
        unsafe {
            sys::log_template_format_with_context(self.wrapped, messages.as_ptr(), messages.len() as i32, options, tz as i32, seq_num, context_id.as_ptr(), self.buffer);
            from_raw_parts((*self.buffer).str as *const u8, (*self.buffer).len)
        }
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
