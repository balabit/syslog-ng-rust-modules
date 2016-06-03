// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate pkg_config;

fn main() {
    let res = pkg_config::find_library("syslog-ng");
    match res {
        Ok(value) => {
            for dir in value.link_paths {
                println!("cargo:rustc-link-search=native={:?}", dir);
            }
            println!("cargo:rustc-link-lib=dylib=syslog-ng");
        },
        Err(err) => {
            println!("libsyslog-ng.so is not found by pkg-config: {}", err);
        }
    }
}
