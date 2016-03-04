// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use syslog_ng_sys::{LogTagId, logmsg};
use syslog_ng_sys::types::*;

use std::collections::BTreeMap;
use std::str;
use std::mem;
use std::slice::from_raw_parts;
use std::ffi::{CStr, CString};

#[cfg(test)]
mod test;

pub struct NVHandle(logmsg::NVHandle);
pub struct LogMessage(*mut logmsg::LogMessage);

impl Drop for LogMessage {
    fn drop(&mut self) {
        unsafe { logmsg::log_msg_unref(self.0) }
    }
}

impl LogMessage {
    pub fn new() -> LogMessage {
        unsafe {
            let msg = logmsg::log_msg_new_empty();
            assert!(msg != ::std::ptr::null_mut());
            LogMessage(msg)
        }
    }

    pub fn wrap_raw(raw: *mut ::sys::LogMessage) -> LogMessage {
        let referenced = unsafe {logmsg::log_msg_ref(raw)};
        LogMessage(referenced)
    }

    unsafe fn c_char_to_str<'a>(value: *const c_char, len: ssize_t) -> &'a str {
        let slce = from_raw_parts(value, len as usize);
        str::from_utf8(mem::transmute(slce)).unwrap()
    }

    pub fn get_value_handle(value_name: &str) -> NVHandle {
        unsafe {
            let name = CString::new(value_name).unwrap();
            NVHandle(logmsg::log_msg_get_value_handle(name.as_ptr()))
        }
    }

    pub fn get<K: Into<NVHandle>>(&self, key: K) -> &str {
        let handle = key.into();
        unsafe {
            let mut size: ssize_t = 0;
            let value = logmsg::__log_msg_get_value(self.0, handle.0, &mut size);
            LogMessage::c_char_to_str(value, size)
        }
    }

    pub fn insert<K: Into<NVHandle>>(&mut self, key: K, value: &str) {
        let handle = key.into();
        unsafe {
            logmsg::log_msg_set_value(self.0, handle.0, value.as_ptr() as *const i8, value.len() as isize);
        }
    }

    pub fn set_tag(&mut self, tag: &str) {
        unsafe {
            let c_tag = CString::new(tag).unwrap();
            logmsg::log_msg_set_tag_by_name(self.0, c_tag.as_ptr());
        }
    }

    pub fn values(&self) -> BTreeMap<String, String> {
        let mut values = BTreeMap::new();
        unsafe {
            let user_data = mem::transmute::<&mut BTreeMap<String, String>,
                                             *mut c_void>(&mut values);
            logmsg::log_msg_values_foreach(self.0, insert_kvpair_to_map, user_data);
        }
        values
    }

    pub fn tags(&self) -> Vec<String> {
        let mut tags = Vec::new();
        unsafe {
            let user_data = mem::transmute::<&mut Vec<String>, *mut c_void>(&mut tags);
            logmsg::log_msg_tags_foreach(self.0, insert_tag_to_vec, user_data);
        }
        tags
    }
}

fn c_char_to_string(value: *const c_char) -> String {
    let bytes = unsafe { CStr::from_ptr(value).to_bytes() };
    let str_slice: &str = str::from_utf8(bytes).unwrap();
    str_slice.to_owned()
}

extern "C" fn insert_tag_to_vec(_: *const logmsg::LogMessage,
                                _: LogTagId,
                                name: *const c_char,
                                user_data: *mut c_void)
                                -> bool {
    unsafe {
        let name = c_char_to_string(name);
        let mut vec: &mut Vec<String> = mem::transmute(user_data);
        vec.push(name);
    }
    false
}

extern "C" fn insert_kvpair_to_map(_: logmsg::NVHandle,
                                   name: *const c_char,
                                   value: *const c_char,
                                   value_len: ssize_t,
                                   user_data: *mut c_void)
                                   -> bool {
    unsafe {
        let name = c_char_to_string(name);
        let value = LogMessage::c_char_to_str(value, value_len).to_string();
        let mut map: &mut BTreeMap<String, String> = mem::transmute(user_data);
        map.insert(name, value);
    }
    false
}

impl<'a> Into<NVHandle> for &'a str {
    fn into(self) -> NVHandle {
        let name = CString::new(self).unwrap();
        let handle = unsafe { logmsg::log_msg_get_value_handle(name.as_ptr()) };
        NVHandle(handle)
    }
}
