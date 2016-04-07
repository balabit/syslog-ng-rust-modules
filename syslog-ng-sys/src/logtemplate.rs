use glib_sys::{GError, GString};

use types::{c_int, c_char, c_void};
use GlobalConfig;
use LogMessage;

pub const LTZ_LOCAL: i32 = 0;
pub const LTZ_SEND: i32 = 1;
pub const LTZ_MAX: i32 = 2;

// this could be expanded but it's best to keep it as an opaque pointer
pub enum LogTemplate {}
// this should be expanded, but log_template_format() can handle a NULL opts
pub enum LogTemplateOptions {}

#[link(name = "syslog-ng")]
extern "C" {
   pub fn log_template_compile(slf: *mut LogTemplate, template: *const c_char, error: *mut *mut GError) -> c_int;
   pub fn log_template_new(cfg: *const GlobalConfig, name: *const c_char) -> *mut LogTemplate;
   pub fn log_template_format(slf: *const LogTemplate,
                              lm: *const LogMessage,
                              opts: *const LogTemplateOptions,
                              tz: c_int,
                              seq_num: i32,
                              context_id: *const c_char,
                              result: *mut GString) -> c_void;
    pub fn log_template_unref(s: *mut LogTemplate);
    pub fn log_template_global_init();
    pub fn log_template_global_deinit();
}
