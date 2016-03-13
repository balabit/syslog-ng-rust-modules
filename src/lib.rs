#[macro_use]
extern crate syslog_ng_common;
#[macro_use]
extern crate log;
#[macro_use]
extern crate cpython;

pub mod py_logmsg;

use std::collections::HashMap;
use std::borrow::Borrow;

use syslog_ng_common::{LogMessage, Parser, ParserBuilder, OptionError};
use cpython::{Python, PyDict, NoArgs, PyBool, PyClone, PyObject};
use cpython::ObjectProtocol; //for call method

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
    options_map: HashMap<String, String>
}

impl ParserBuilder for PythonParserBuilder {
    type Parser = PythonParser;
    fn new() -> Self {
        PythonParserBuilder {
            module: None,
            class: None,
            options_map: HashMap::new()
        }
    }
    fn option(&mut self, name: String, value: String) {
        match name.borrow() {
            options::MODULE => { self.module = Some(value); },
            options::CLASS => { self.class = Some(value); },
            _ => { self.options_map.insert(name, value); }
        }
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        let gil = Python::acquire_gil();
        let py = gil.python(); // obtain `Python` token

        match (self.module, self.class) {
            (Some(ref module_name), Some(ref class_name)) => {
                debug!("Trying to load Python module, module='{}'", module_name);
                let module = py.import(module_name).unwrap();
                debug!("Trying to load Python class, class='{}'", class_name);
                let class = module.get(py, class_name).unwrap();
                debug!("Trying to instantiate Python parser");
                let parser_instance = class.call(py, NoArgs, None).unwrap();
                debug!("Instantiating the options dict");
                let options = PyDict::new(py);
                for (k, v) in self.options_map {
                    debug!("Adding values to the options dict, key='{}', value='{}'", &k, &v);
                    options.set_item(py, k, v).unwrap();
                }
                debug!("Trying to call init() on the Python parser instance");
                let init_result = parser_instance.call_method(py, "init", (&options, ), None).unwrap();
                debug!("Trying to check the resulf ot init()");
                let as_bool = init_result.cast_into::<PyBool>(py).unwrap();
                if as_bool.is_true() {
                    debug!("Python parser successfully initialized, class='{}'", &class_name);
                    Ok(PythonParser {parser: parser_instance})
                } else {
                    error!("Failed to initialize Python parser, class='{}'", &class_name);
                    Err(OptionError::missing_required_option("asdas"))
                }
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
