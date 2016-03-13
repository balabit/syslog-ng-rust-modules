extern crate python_parser;
extern crate syslog_ng_common;
extern crate cpython;
extern crate env_logger;

use std::env;
use python_parser::{PythonParserBuilder, options, PythonParser, PyLogMessage};
use syslog_ng_common::{ParserBuilder, LogMessage, Parser};
use cpython::{Python, PyResult, PyObject};

const TEST_MODULE_NAME: &'static str = "_test_module";

fn build_parser(module_name: &str, class_name: &str) -> PythonParser {
    let mut builder = PythonParserBuilder::new();
    builder.option(options::MODULE.to_owned(), module_name.to_owned());
    builder.option(options::CLASS.to_owned(), class_name.to_owned());
    builder.build().unwrap()
}

fn call_parse<'p>(py: Python<'p>, module_name: &str, class_name: &str) -> PyResult<PyObject> {
    let mut parser = build_parser(module_name, class_name);
    let logmsg = LogMessage::new();
    let pylogmsg = PyLogMessage::new(py, logmsg).unwrap();
    parser.process_parsing(py, pylogmsg, "input message to be parsed")
}

#[test]
fn test_parse_method_can_be_called() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let _ = call_parse(py, TEST_MODULE_NAME, "ParserClassWithGoodParseMethod").unwrap();
}

#[test]
fn test_error_is_returned_if_there_is_no_parse_method() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let result = call_parse(py, TEST_MODULE_NAME, "ParseMethodReturnsNotBoolean").unwrap();
    let _ = PythonParser::process_parse_result(py, result).err().unwrap();
}

#[test]
fn test_parse_method_which_returns_boolean_does_not_raise_errors() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let result = call_parse(py, TEST_MODULE_NAME, "ParserClassWithGoodParseMethod").unwrap();
    let _ = PythonParser::process_parse_result(py, result).unwrap();
}

#[test]
fn test_successful_parse() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let mut parser = build_parser(TEST_MODULE_NAME, "ParseReturnsTrue");
    let mut logmsg = LogMessage::new();
    assert_eq!(true, parser.parse(&mut logmsg, "input message to be parsed"));
}

#[test]
fn test_unsucessful_parse() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let mut parser = build_parser(TEST_MODULE_NAME, "ParseReturnsFalse");
    let mut logmsg = LogMessage::new();
    assert_eq!(false, parser.parse(&mut logmsg, "input message to be parsed"));
}

#[test]
fn test_parse_method_raises_an_exception() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let mut parser = build_parser(TEST_MODULE_NAME, "ExceptionIsRaisedInParseMethod");
    let mut logmsg = LogMessage::new();
    assert_eq!(false, parser.parse(&mut logmsg, "input message to be parsed"));
}
