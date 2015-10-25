extern crate syslog_ng_build;

fn main() {
    let canonical_name = "actiondb-parser";
    let description = "Efficient log parser";
    let parser_name = "actiondb-rs";
    syslog_ng_build::create_module(canonical_name, description, Some(parser_name));
}
