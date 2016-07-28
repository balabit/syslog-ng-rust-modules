use GlobalConfig;

use syslog_ng_sys;

use std::ffi::CString;

/// Represents a syslog-ng plugin.
pub struct Plugin;

/// Loads the plugin with `name`.
///
/// Returns `true` on successful loading, otherwise `false`.
///
/// # Panics
///
/// `name` must not contain any `\0` bytes.
impl Plugin {
    pub fn load_module(name: &str, cfg: &mut GlobalConfig) -> bool {
        let name = CString::new(name).unwrap();
        let result =  unsafe {
            syslog_ng_sys::plugin::plugin_load_module(name.as_ptr(),
                                                      cfg.raw_ptr(),
                                                      ::std::ptr::null_mut()) };
        if result > 0 {
            true
        } else {
            false
        }
    }
}
