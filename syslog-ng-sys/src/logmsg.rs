use ::types::*;

pub enum LogMessage {}
pub type NVHandle = u32;
pub type LogTagId = u16;
pub type NVTableForeachFunc = extern "C" fn(// handle:
                                            NVHandle, // name:
                                            *const c_char, // value:
                                            *const c_char, // value_len:
                                            ssize_t, // user_data:
                                            *mut c_void)
                                            -> bool;
pub type LogMessageTagsForeachFunc = extern "C" fn(// msg:
                                                   *const LogMessage, // tag_id:
                                                   LogTagId, // name:
                                                   *const c_char, // user_data:
                                                   *mut c_void)
                                                   -> bool;

#[link(name = "syslog-ng")]
extern "C" {
    pub fn log_msg_unref(m: *mut LogMessage) -> ();
    pub fn log_msg_get_value_handle(value_name: *const c_char) -> NVHandle;
    pub fn __log_msg_get_value(m: *const LogMessage,
                               handle: NVHandle,
                               value_len: *mut ssize_t)
                               -> *const c_char;
    pub fn __log_msg_get_value_by_name(m: *const LogMessage,
                                       name: *const c_char,
                                       value_len: *mut ssize_t)
                                       -> *const c_char;
    pub fn __log_msg_set_value_by_name(msg: *mut LogMessage,
                                       name: *const c_char,
                                       value: *const c_char,
                                       value_length: ssize_t);
    pub fn log_msg_set_tag_by_name(msg: *mut LogMessage, name: *const c_char);
    pub fn log_msg_values_foreach(msg: *const LogMessage,
                                  func: NVTableForeachFunc,
                                  user_data: *mut c_void);
    pub fn log_msg_new_empty() -> *mut LogMessage;
    pub fn log_msg_tags_foreach(msg: *const LogMessage,
                                callback: LogMessageTagsForeachFunc,
                                user_data: *mut c_void);
    pub fn log_msg_registry_init();
    pub fn log_msg_registry_deinit();
}
