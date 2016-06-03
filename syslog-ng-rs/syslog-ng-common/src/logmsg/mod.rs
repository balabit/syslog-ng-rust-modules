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
use std::mem;
use std::slice::from_raw_parts;
use std::ffi::{CStr, CString};

#[cfg(test)]
mod test;

pub struct NVHandle(logmsg::NVHandle);
pub struct LogMessage(pub *mut logmsg::LogMessage);

impl Drop for LogMessage {
    fn drop(&mut self) {
        unsafe { logmsg::log_msg_unref(self.0) }
    }
}

unsafe impl Send for LogMessage {}

impl Clone for LogMessage {
    fn clone(&self) -> LogMessage {
        LogMessage::wrap_raw(self.0)
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

    pub fn into_raw(self) -> *mut ::sys::LogMessage {
        // self will be destroyed by the end of this functions so we need to
        // increment the refcount
        unsafe {logmsg::log_msg_ref(self.0)}
    }

    pub fn get_value_handle(value_name: &str) -> NVHandle {
        unsafe {
            let name = CString::new(value_name).unwrap();
            NVHandle(logmsg::log_msg_get_value_handle(name.as_ptr()))
        }
    }

    pub fn get<K: Into<NVHandle>>(&self, key: K) -> Option<&[u8]> {
        let handle = key.into();
        let mut size: ssize_t = 0;
        let value = unsafe { logmsg::__log_msg_get_value(self.0, handle.0, &mut size) };
        if size > 0 {
            let value = unsafe { from_raw_parts(value as *const u8, size as usize) };
            Some(value)
        } else {
            None
        }
    }

    pub fn insert<K: Into<NVHandle>>(&mut self, key: K, value: &[u8]) {
        let handle = key.into();
        unsafe {
            logmsg::log_msg_set_value(self.0, handle.0, value.as_ptr() as *const i8, value.len() as isize);
        }
    }

    pub fn set_tag(&mut self, tag: &[u8]) {
        let c_tag = CString::new(tag).unwrap();
        unsafe {
            logmsg::log_msg_set_tag_by_name(self.0, c_tag.as_ptr());
        }
    }

    pub fn values(&self) -> BTreeMap<Vec<u8>, Vec<u8>> {
        let mut values = BTreeMap::new();
        unsafe {
            let user_data = mem::transmute::<&mut BTreeMap<Vec<u8>, Vec<u8>>,
                                             *mut c_void>(&mut values);
            logmsg::log_msg_values_foreach(self.0, insert_kvpair_to_map, user_data);
        }
        values
    }

    pub fn tags(&self) -> Vec<Vec<u8>> {
        let mut tags = Vec::new();
        unsafe {
            let user_data = mem::transmute::<&mut Vec<Vec<u8>>, *mut c_void>(&mut tags);
            logmsg::log_msg_tags_foreach(self.0, insert_tag_to_vec, user_data);
        }
        tags
    }
}

extern "C" fn insert_tag_to_vec(_: *const logmsg::LogMessage,
                                _: LogTagId,
                                name: *const c_char,
                                user_data: *mut c_void)
                                -> bool {
    unsafe {
        let bytes = CStr::from_ptr(name).to_bytes().to_vec();
        let mut vec: &mut Vec<Vec<u8>> = mem::transmute(user_data);
        vec.push(bytes);
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
        let name = CStr::from_ptr(name).to_bytes().to_vec();
        let value = from_raw_parts(value as *const u8, value_len as usize).to_vec();
        let mut map: &mut BTreeMap<Vec<u8>, Vec<u8>> = mem::transmute(user_data);
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

impl<'a> Into<NVHandle> for &'a [u8] {
    fn into(self) -> NVHandle {
        let mut name = Vec::from(self);

        let has_trailing_zero = name.last().map_or(false, |last| *last == 0);

        if !has_trailing_zero {
            name.push(0 as u8);
        }

        let handle = unsafe { logmsg::log_msg_get_value_handle(name.as_ptr() as *const i8) };
        NVHandle(handle)
    }
}
