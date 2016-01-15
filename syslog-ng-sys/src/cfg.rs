use ::types::*;

pub enum GlobalConfig {}

#[link(name = "syslog-ng")]
extern "C" {
    pub fn cfg_get_user_version(cfg: *const GlobalConfig) -> c_int;
    pub fn cfg_get_parsed_version(cfg: *const GlobalConfig) -> c_int;
    pub fn cfg_get_filename(cfg: *const GlobalConfig) -> *const c_char;
}
