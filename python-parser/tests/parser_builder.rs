extern crate python_parser;
extern crate syslog_ng_common;
extern crate cpython;
extern crate env_logger;

use std::env;
use python_parser::{PythonParserBuilder, options};
use syslog_ng_common::{ParserBuilder, SYSLOG_NG_INITIALIZED, syslog_ng_global_init, GlobalConfig};
use syslog_ng_common::mock::MockPipe;
use cpython::{Python, PyDict};

const TEST_MODULE_NAME: &'static str = "_test_module";

#[test]
fn test_exising_module_can_be_imported() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let _ = PythonParserBuilder::<MockPipe>::load_module(py, TEST_MODULE_NAME).unwrap();
}

#[test]
fn test_non_exising_module_cannot_be_imported() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let _ = PythonParserBuilder::<MockPipe>::load_module(py, "__non_existing_python_module_name").err().unwrap();
}

#[test]
fn test_existing_class_be_imported_from_module() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let module = PythonParserBuilder::<MockPipe>::load_module(py, TEST_MODULE_NAME).unwrap();
    let _ = PythonParserBuilder::<MockPipe>::load_class(py, &module, "ExistingParser").unwrap();
}

#[test]
fn test_non_exising_class_cannot_be_imported() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let module = PythonParserBuilder::<MockPipe>::load_module(py, TEST_MODULE_NAME).unwrap();
    let _ = PythonParserBuilder::<MockPipe>::load_class(py, &module, "NonExistingParser").err().unwrap();
}

#[test]
fn test_parser_class_is_callable() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let module = PythonParserBuilder::<MockPipe>::load_module(py, TEST_MODULE_NAME).unwrap();
    let class = PythonParserBuilder::<MockPipe>::load_class(py, &module, "CallableClass").unwrap();
    let _ = PythonParserBuilder::<MockPipe>::instantiate_class(py, &class).unwrap();
}

#[test]
fn test_not_callable_object_cannot_be_instantiated() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let module = PythonParserBuilder::<MockPipe>::load_module(py, TEST_MODULE_NAME).unwrap();
    let class = PythonParserBuilder::<MockPipe>::load_class(py, &module, "NotCallableObject").unwrap();
    let _ = PythonParserBuilder::<MockPipe>::instantiate_class(py, &class).err().unwrap();
}

#[test]
fn test_init_is_called_if_it_exists() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let module = PythonParserBuilder::<MockPipe>::load_module(py, TEST_MODULE_NAME).unwrap();
    let class = PythonParserBuilder::<MockPipe>::load_class(py, &module, "ClassWithInitMethod").unwrap();
    let instance = PythonParserBuilder::<MockPipe>::instantiate_class(py, &class).unwrap();
    let _ = PythonParserBuilder::<MockPipe>::initialize_instance(py, &instance, PyDict::new(py)).unwrap();
}

#[test]
fn test_parser_may_not_have_init_method() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let module = PythonParserBuilder::<MockPipe>::load_module(py, TEST_MODULE_NAME).unwrap();
    let class = PythonParserBuilder::<MockPipe>::load_class(py, &module, "InitMethodReturnsNotNone").unwrap();
    let instance = PythonParserBuilder::<MockPipe>::instantiate_class(py, &class).unwrap();
    let _ = PythonParserBuilder::<MockPipe>::initialize_instance(py, &instance, PyDict::new(py)).err().unwrap();
}

#[test]
fn test_init_must_return_nothing() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let module = PythonParserBuilder::<MockPipe>::load_module(py, TEST_MODULE_NAME).unwrap();
    let class = PythonParserBuilder::<MockPipe>::load_class(py, &module, "ParserWithoutInitMethod").unwrap();
    let instance = PythonParserBuilder::<MockPipe>::instantiate_class(py, &class).unwrap();
    let _ = PythonParserBuilder::<MockPipe>::initialize_instance(py, &instance, PyDict::new(py)).unwrap();
}

#[test]
fn test_module_loading_and_class_initialization() {
    let _ = env_logger::init();
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    let gil = Python::acquire_gil();
    let py = gil.python();
    let options = [];
    let _ = PythonParserBuilder::<MockPipe>::load_and_init_class(py, "__non_existing_python_module_name", "ExistingParser", &options).err().unwrap();
    let _ = PythonParserBuilder::<MockPipe>::load_and_init_class(py, TEST_MODULE_NAME, "NonExistingParser", &options).err().unwrap();
    let _ = PythonParserBuilder::<MockPipe>::load_and_init_class(py, TEST_MODULE_NAME, "ExistingParser", &options).unwrap();
    let _ = PythonParserBuilder::<MockPipe>::load_and_init_class(py, TEST_MODULE_NAME, "ClassWithInitMethod", &options).unwrap();
    let _ = PythonParserBuilder::<MockPipe>::load_and_init_class(py, TEST_MODULE_NAME, "InitMethodReturnsNotNone", &options).err().unwrap();
}

#[test]
fn test_parser_can_be_built_if_there_is_no_error() {
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let cfg = GlobalConfig::new(0x0308);
    let mut builder = PythonParserBuilder::<MockPipe>::new(cfg);
    builder.option(options::MODULE.to_owned(), "_test_module".to_owned());
    builder.option(options::CLASS.to_owned(), "ExistingParser".to_owned());
    let _ = builder.build().unwrap();
}

#[test]
fn test_parser_cannot_be_built_if_there_is_an_error() {
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let cfg = GlobalConfig::new(0x0308);
    let mut builder = PythonParserBuilder::<MockPipe>::new(cfg);
    builder.option(options::MODULE.to_owned(), "_test_module".to_owned());
    builder.option(options::CLASS.to_owned(), "NonExistingParser".to_owned());
    let _ = builder.build().err().unwrap();
}

#[test]
fn test_exception_is_raised_in_init_method() {
    env::set_var("PYTHONPATH", env::current_dir().unwrap());
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe { syslog_ng_global_init(); }
    });
    let cfg = GlobalConfig::new(0x0308);
    let mut builder = PythonParserBuilder::<MockPipe>::new(cfg);
    builder.option(options::MODULE.to_owned(), "_test_module".to_owned());
    builder.option(options::CLASS.to_owned(), "ExceptionIsRaisedInInitMethod".to_owned());
    let _ = builder.build().err().unwrap();
}
