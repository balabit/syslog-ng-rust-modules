// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use ::types::*;

pub enum GlobalConfig {}

#[link(name = "syslog-ng")]
extern "C" {
    pub fn cfg_get_user_version(cfg: *const GlobalConfig) -> c_int;
    pub fn cfg_get_parsed_version(cfg: *const GlobalConfig) -> c_int;
    pub fn cfg_get_filename(cfg: *const GlobalConfig) -> *const c_char;
}
