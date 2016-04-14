// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use syslog_ng_sys::cfg;
use std::ffi::CStr;

use SYSLOG_NG_INITIALIZED;
use syslog_ng_global_init;

enum InternalState {
    Owned(*mut cfg::GlobalConfig),
    Borrowed(*mut cfg::GlobalConfig),
}

pub struct GlobalConfig(InternalState);

impl GlobalConfig {
    pub fn new(version: i32) -> GlobalConfig {
        let cfg = unsafe { cfg::cfg_new(version) };
        GlobalConfig(InternalState::Owned(cfg))
    }

    pub fn borrow(cfg: *mut cfg::GlobalConfig) -> GlobalConfig {
        GlobalConfig(InternalState::Borrowed(cfg))
    }

    pub fn get_user_version(&self) -> (u8, u8) {
        let ptr = self.raw_ptr();
        let mut version = unsafe { cfg::cfg_get_user_version(ptr) };

        if version < 0 {
            error!("User config version must be greater than 0, using 0 as version");
            version = 0;
        }

        convert_version(version as u16)
    }

    pub fn get_parsed_version(&self) -> (u8, u8) {
        let ptr = self.raw_ptr();
        let mut version = unsafe { cfg::cfg_get_parsed_version(ptr) };

        if version < 0 {
            error!("Parsed config version must be greater than 0, using 0 as version");
            version = 0;
        }

        convert_version(version as u16)
    }

    pub fn get_filename(&self) -> &CStr {
        let ptr = self.raw_ptr();
        unsafe { CStr::from_ptr(cfg::cfg_get_filename(ptr)) }
    }

    pub fn raw_ptr(&self) -> *mut cfg::GlobalConfig {
        match self.0 {
            InternalState::Owned(ptr) => ptr,
            InternalState::Borrowed(ptr) => ptr,
        }
    }
}

impl Drop for GlobalConfig {
    fn drop(&mut self) {
        if let InternalState::Owned(ptr) = self.0 {
            unsafe { cfg::cfg_free(ptr) };
        }
    }
}

fn hex_to_dec(hex: u8) -> u8 {
    let mut dec = 0;
    let mut shifted_hex = hex;

    for i in 0..2 {
        dec += (shifted_hex % 16) * 10u8.pow(i);
        shifted_hex >>= 4;
    }

    dec
}

fn convert_version(version: u16) -> (u8, u8) {
    let minor = hex_to_dec(version as u8);
    let major = hex_to_dec((version >> 8) as u8);
    (major, minor)
}

#[test]
fn one_digit_hex_number_when_converted_to_decimal_works() {
    let dec = hex_to_dec(0x3);
    assert_eq!(dec, 3);
}

#[test]
fn more_digits_hex_number_when_converted_to_decimal_works() {
    let dec = hex_to_dec(0x22);
    assert_eq!(dec, 22);
}

#[test]
fn hex_version_when_converted_to_minor_version_works() {
    let version = 0x0316;

    let (_, minor) = convert_version(version);
    assert_eq!(minor, 16);
}

#[test]
fn hex_version_when_converted_to_major_version_works() {
    let version = 0x0316;

    let (major, _) = convert_version(version);
    assert_eq!(major, 3);
}

#[test]
fn test_borrowed_configuration_is_not_freed_on_destruction() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let owned = GlobalConfig::new(0x0308);
    let _ = GlobalConfig::borrow(owned.raw_ptr());
}
