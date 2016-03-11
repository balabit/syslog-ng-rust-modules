extern crate syslog_ng_build;

fn main() {
    let canonical_name = "regex-parser";
    let description = "This is regex based log parser written in Rust";
    let parser_name = "regex-rs";
    syslog_ng_build::create_module(canonical_name, description, Some(parser_name));
}
