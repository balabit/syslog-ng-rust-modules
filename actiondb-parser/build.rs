// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate syslog_ng_build;

fn main() {
    let canonical_name = "actiondb-parser";
    let description = "Efficient log parser";
    let parser_name = "actiondb-rs";
    syslog_ng_build::create_module(canonical_name, description, Some(parser_name));
}
