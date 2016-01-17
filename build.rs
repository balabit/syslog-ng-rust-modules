extern crate syslog_ng_build;

fn main() {
    let canonical_name = "correlation-parser";
    let description = "Log correlation";
    let parser_name = "correlation-rs";
    syslog_ng_build::create_module(canonical_name, description, Some(parser_name));
}
