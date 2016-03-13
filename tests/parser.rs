extern crate python_parser;
extern crate syslog_ng_common;
extern crate env_logger;

use std::env;
use python_parser::{PythonParserBuilder, options};
use syslog_ng_common::{ParserBuilder, Parser, LogMessage};
use syslog_ng_common::sys::logmsg::log_msg_registry_init;

#[test]
fn test_parser_module_ca_be_imported() {
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let mut builder = PythonParserBuilder::new();
    builder.option(options::MODULE.to_owned(), "_test_module".to_owned());
    builder.option(options::CLASS.to_owned(), "ParserForImport".to_owned());
    let _ = builder.build().unwrap();
}

#[test]
fn test_parser_parses_the_message() {
    unsafe {
        log_msg_registry_init();
    };
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let mut builder = PythonParserBuilder::new();
    builder.option(options::MODULE.to_owned(), "_test_module".to_owned());
    builder.option(options::CLASS.to_owned(), "ParserForImport".to_owned());
    let mut parser = builder.build().unwrap();
    let mut logmsg = LogMessage::new();
    let _ = parser.parse(&mut logmsg, "input message for parse method");
}
