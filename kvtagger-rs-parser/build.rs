extern crate syslog_ng_build;

fn main() {
    let canonical_name = "kvtagger-parser";
    let description = "This is a key-value tagger written in Rust";
    let parser_name = "kvtagger-rs";
    syslog_ng_build::create_module(canonical_name, description, Some(parser_name));
}
