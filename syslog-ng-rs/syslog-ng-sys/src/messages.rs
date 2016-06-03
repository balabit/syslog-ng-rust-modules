// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use ::types::*;

pub enum EVTREC {}

#[link(name = "syslog-ng")]
extern "C" {
    pub fn msg_event_create_from_desc(prio: i32, desc: *const c_char) -> *mut EVTREC;
    pub fn msg_event_suppress_recursions_and_send(e: *mut EVTREC);
    pub static debug_flag: c_int;
    pub static trace_flag: c_int;
}
