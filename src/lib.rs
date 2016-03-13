#[macro_use]
extern crate syslog_ng_common;
#[macro_use]
extern crate log;
#[macro_use]
extern crate cpython;

pub mod py_logmsg;

use std::borrow::Borrow;

use syslog_ng_common::{LogMessage, Parser, ParserBuilder, OptionError};
use cpython::{Python, PyDict, NoArgs, PyClone, PyObject, PyResult, PyModule, PyErr, PyString};
use cpython::ObjectProtocol; //for call method
use cpython::exc::TypeError;

use py_logmsg::PyLogMessage;

pub mod options {
    pub const MODULE: &'static str = "module";
    pub const CLASS: &'static str = "class";
}

pub struct PythonParser {
    parser: PyObject
}

impl Clone for PythonParser {
    fn clone(&self) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python(); // obtain `Python` token
        PythonParser {parser: self.parser.clone_ref(py)}
    }
}

pub struct PythonParserBuilder {
    module: Option<String>,
    class: Option<String>,
    options: Vec<(String, String)>
}

impl PythonParserBuilder {
    // Although these functions are very small ones, they are very useful for testing
    pub fn load_module<'p>(py: Python<'p>, module_name: &str) -> PyResult<PyModule> {
        debug!("Trying to load Python module, module='{}'", module_name);
        py.import(module_name)
    }
    pub fn load_class<'p>(py: Python<'p>, module: &PyModule,  class_name: &str) -> PyResult<PyObject> {
        debug!("Trying to load Python class, class='{}'", class_name);
        module.get(py, class_name)
    }
    pub fn instantiate_class<'p>(py: Python<'p>, class: &PyObject) -> PyResult<PyObject> {
        debug!("Trying to instantiate Python parser");
        class.call(py, NoArgs, None)
    }
    pub fn create_options_dict<'p>(py: Python<'p>, init_options: &[(String, String)]) -> PyResult<PyDict> {
        debug!("Instantiating the options dict");
        let options = PyDict::new(py);
        for &(ref k, ref v) in init_options {
            debug!("Adding values to the options dict, key='{}', value='{}'", k, v);
            try!(options.set_item(py, k, v));
        }
        Ok(options)
    }
    fn call_init<'p>(py: Python<'p>, instance: &PyObject, options: PyDict) -> PyResult<()> {
        let init_result = try!(instance.call_method(py, "init", (&options, ), None));
        if init_result == Python::None(py) {
            Ok(())
        } else {
            let errmsg = PyString::new(py, "The init() method mustn't return any value");
            Err(PyErr::new::<TypeError, PyString>(py, errmsg))
        }
    }
    pub fn initialize_instance<'p>(py: Python<'p>, instance: &PyObject, options: PyDict) -> PyResult<()> {
        debug!("Trying to call init() on the Python parser instance");
        if try!(instance.hasattr(py, "init")) {
            Self::call_init(py, instance, options)
        } else {
            Ok(())
        }
    }
    pub fn initialize_class<'p>(py: Python<'p>, class: &PyObject, options: &[(String, String)]) -> PyResult<PyObject> {
        let parser_instance = try!(Self::instantiate_class(py, &class));
        let options = try!(Self::create_options_dict(py, options));
        let _ = try!(Self::initialize_instance(py, &parser_instance, options));
        Ok(parser_instance)
    }

    pub fn load_and_init_class<'p>(py: Python<'p>, module_name: &str, class_name: &str, options: &[(String, String)]) -> PyResult<PyObject> {
        let module = try!(Self::load_module(py, module_name));
        let class = try!(Self::load_class(py, &module, class_name));
        Self::initialize_class(py, &class, options)
    }
}

impl ParserBuilder for PythonParserBuilder {
    type Parser = PythonParser;
    fn new() -> Self {
        PythonParserBuilder {
            module: None,
            class: None,
            options: Vec::new()
        }
    }
    fn option(&mut self, name: String, value: String) {
        match name.borrow() {
            options::MODULE => { self.module = Some(value); },
            options::CLASS => { self.class = Some(value); },
            _ => { self.options.push((name, value)); }
        }
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        let gil = Python::acquire_gil();
        let py = gil.python(); // obtain `Python` token

        match (self.module, self.class) {
            (Some(ref module_name), Some(ref class_name)) => {
                let parser_instance = PythonParserBuilder::load_and_init_class(py, module_name, class_name, &self.options).unwrap();
                debug!("Python parser successfully initialized, class='{}'", &class_name);
                Ok(PythonParser {parser: parser_instance})
            },
            (ref module, ref class) => {
                error!("Missing parameters in Python parser: module={:?}, class={:?}", module, class);
                Err(OptionError::missing_required_option("module"))
            }
        }
    }
}

impl Parser for PythonParser {
    fn parse(&mut self, logmsg: &mut LogMessage, input: &str) -> bool {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let pylogmsg = PyLogMessage::new(py, logmsg.clone());
        debug!("Trying to call parse() method on Python parser");
        let result = self.parser.call_method(py, "parse", (pylogmsg, input), None).unwrap();
        debug!("Trying to check the result of parse()");
        result.extract::<bool>(py).unwrap()
    }
}

parser_plugin!(PythonParserBuilder);
