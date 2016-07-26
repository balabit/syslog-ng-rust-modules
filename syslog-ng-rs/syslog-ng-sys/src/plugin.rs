use types::{c_char, c_int};
use GlobalConfig;

pub enum Plugin {}
pub enum CfgArgs {}

#[link(name = "syslog-ng")]
extern "C" {
    pub fn plugin_load_module(module_name: *const c_char,
                              cfg: *mut GlobalConfig,
                              cfg_args: *mut CfgArgs)
                              -> c_int;
}
