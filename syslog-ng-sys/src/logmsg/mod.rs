use types::*;

use std::collections::BTreeMap;
use std::str;
use std::mem;
use std::slice::from_raw_parts;
use std::ffi::{
    CStr,
    CString
};

mod ffi;
#[cfg(test)]
mod test;

use self::ffi::LogTagId;

pub use self::ffi::LogMessage;
pub use self::ffi::NVHandle;

impl Drop for LogMessage {

    fn drop(&mut self) {
        unsafe {
            ffi::log_msg_unref(self)
        }
    }
}

impl LogMessage {
    pub fn new() -> *mut LogMessage {
        unsafe {
            ffi::log_msg_new_empty()
        }
    }

    unsafe fn c_char_to_str<'a>(value: *const c_char, len: ssize_t) -> &'a str {
        let slce = from_raw_parts(value, len as usize);
        str::from_utf8(mem::transmute(slce)).unwrap()
    }

    pub fn get_value_handle(value_name: &str) -> NVHandle {
        unsafe {
            let name = CString::new(value_name).unwrap();
            ffi::log_msg_get_value_handle(name.as_ptr())
        }
    }

    pub fn get_value_by_name(&self, value_name: &str) -> &str {
        unsafe {
            let name = CString::new(value_name).unwrap();
            let mut size: ssize_t = 0;
            let value = ffi::__log_msg_get_value_by_name(&*self, name.as_ptr(), &mut size);
            LogMessage::c_char_to_str(value, size)
        }
    }

    pub fn get_value(&self, handle: NVHandle) -> &str {
        unsafe {
            let mut size: ssize_t = 0;
            let value = ffi::__log_msg_get_value(&*self, handle, &mut size);
            LogMessage::c_char_to_str(value, size)
        }
    }

    pub fn set_value(&mut self, key: &str, value: &str) {
        unsafe {
            let c_key = CString::new(key).unwrap();
            let c_value = CString::new(value).unwrap();
            ffi::__log_msg_set_value_by_name(&mut *self, c_key.as_ptr(), c_value.as_ptr(), value.len() as i64);
        }
    }

    pub fn set_tag(&mut self, tag: &str) {
        unsafe {
            let c_tag = CString::new(tag).unwrap();
            ffi::log_msg_set_tag_by_name(&mut *self, c_tag.as_ptr());
        }
    }

    pub fn values(&self) -> BTreeMap<String, String> {
        let mut values = BTreeMap::new();
        unsafe {
            let user_data = mem::transmute::<&mut BTreeMap<String, String>, *mut c_void>(&mut values);
            ffi::log_msg_values_foreach(&*self, insert_kvpair_to_map, user_data);
        }
        values
    }

    pub fn tags(&self) -> Vec<String> {
        let mut tags = Vec::new();
        unsafe {
            let user_data = mem::transmute::<&mut Vec<String>, *mut c_void>(&mut tags);
            ffi::log_msg_tags_foreach(&*self, insert_tag_to_vec, user_data);
        }
        tags
    }
}

fn c_char_to_string(value: *const c_char) -> String {
    let bytes = unsafe {
        CStr::from_ptr(value).to_bytes()
    };
    let str_slice: &str = str::from_utf8(bytes).unwrap();
    str_slice.to_owned()
}

extern fn insert_tag_to_vec(_: *const LogMessage, _: LogTagId, name: *const c_char, user_data: *mut c_void) -> bool {
    unsafe {
        let name = c_char_to_string(name);
        let mut vec: &mut Vec<String> = mem::transmute(user_data);
        vec.push(name);
    }
    false
}

extern fn insert_kvpair_to_map(_: NVHandle, name: *const c_char, value: *const c_char, value_len: ssize_t, user_data: *mut c_void) -> bool {
    unsafe {
        let name = c_char_to_string(name);
        let value = LogMessage::c_char_to_str(value, value_len).to_string();
        let mut map: &mut BTreeMap<String, String> = mem::transmute(user_data);
        map.insert(name, value);
    }
    false
}
