extern crate syslog_ng_build;

fn main() {
    let canonical_name = "python-parser";
    let description = "This is a Python parser written in Rust";
    let parser_name = "python-rs";
    syslog_ng_build::create_module(canonical_name, description, Some(parser_name));
}
