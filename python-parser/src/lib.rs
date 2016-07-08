#[macro_use]
extern crate syslog_ng_common;
#[macro_use]
extern crate log;
#[macro_use]
extern crate cpython;

pub mod py_logmsg;
pub mod utils;

use std::borrow::Borrow;
use std::marker::PhantomData;

use syslog_ng_common::{LogMessage, Parser, ParserBuilder, Error, Pipe, GlobalConfig};
use cpython::{Python, PyDict, NoArgs, PyObject, PyResult, PyModule, PyErr, PyString, ToPyObject};
use cpython::ObjectProtocol; //for call method
use cpython::exc::TypeError;

pub use py_logmsg::PyLogMessage;

pub mod options {
    pub const MODULE: &'static str = "module";
    pub const CLASS: &'static str = "class";
}

pub struct PythonParser<P: Pipe> {
    parser: PyObject,
    _marker: PhantomData<P>
}

pub struct PythonParserBuilder<P: Pipe> {
    module: Option<String>,
    class: Option<String>,
    options: Vec<(String, String)>,
    _marker: PhantomData<P>
}

impl<P: Pipe> PythonParserBuilder<P> {
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
        let mut dict = module.dict(py);

        try!(python_register_callbacks(py, &mut dict));

        let class = try!(Self::load_class(py, &module, class_name));
        Self::initialize_class(py, &class, options)
    }
}

fn python_register_callbacks(py: Python, dict: &mut PyDict) -> PyResult<()> {
    try!(python_register_callback(py, dict, "error", py_fn!(python_error_callback(error_message: &str))));
    try!(python_register_callback(py, dict, "info", py_fn!(python_info_callback(info_message: &str))));
    try!(python_register_callback(py, dict, "trace", py_fn!(python_trace_callback(trace_message: &str))));
    try!(python_register_callback(py, dict, "warning", py_fn!(python_warning_callback(warning_message: &str))));
    try!(python_register_callback(py, dict, "debug", py_fn!(python_debug_callback(debug_message: &str))));
    Ok(())
}

fn python_register_callback<F: ToPyObject>(py: Python, dict: &mut PyDict, name: &str, function: F) -> PyResult<()> {
    if try!(dict.contains(py, name)) {
        warn!("Already implemented {}() function, omitting callback definition.", name);
    } else {
        try!(dict.set_item(py, name, function));
    }
    Ok(())
}

fn python_error_callback(_: Python, error_message: &str) -> PyResult<NoArgs> {
    error!("{}", error_message);
    Ok(NoArgs)
}

fn python_info_callback(_: Python, info_message: &str) -> PyResult<NoArgs> {
    info!("{}", info_message);
    Ok(NoArgs)
}

fn python_trace_callback(_: Python, trace_message: &str) -> PyResult<NoArgs> {
    trace!("{}", trace_message);
    Ok(NoArgs)
}

fn python_warning_callback(_: Python, warning_message: &str) -> PyResult<NoArgs> {
    warn!("{}", warning_message);
    Ok(NoArgs)
}

fn python_debug_callback(_: Python, debug_message: &str) -> PyResult<NoArgs> {
    debug!("{}", debug_message);
    Ok(NoArgs)
}

impl<P: Pipe> Clone for PythonParserBuilder<P> {
    fn clone(&self) -> Self {
        PythonParserBuilder {
            module: self.module.clone(),
            class: self.class.clone(),
            options: self.options.clone(),
            _marker: PhantomData
        }
    }
}

impl<P: Pipe> ParserBuilder<P> for PythonParserBuilder<P> {
    type Parser = PythonParser<P>;
    fn new(_: GlobalConfig) -> Self {
        PythonParserBuilder {
            module: None,
            class: None,
            options: Vec::new(),
            _marker: PhantomData
        }
    }
    fn option(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.borrow() {
            options::MODULE => {
                self.module = Some(value);
            },
            options::CLASS => {
                self.class = Some(value);
            },
            _ => {
                self.options.push((name, value));
            }
        }
        Ok(())
    }
    fn build(self) -> Result<Self::Parser, Error> {
        let gil = Python::acquire_gil();
        let py = gil.python(); // obtain `Python` token

        let module_name = try!(self.module.ok_or(Error::missing_required_option(options::MODULE)));
        let class_name = try!(self.class.ok_or(Error::missing_required_option(options::CLASS)));

        match PythonParserBuilder::<P>::load_and_init_class(py, &module_name, &class_name, &self.options) {
            Ok(parser_instance) => {
                debug!("Python parser successfully initialized, class='{}'", &class_name);
                Ok(PythonParser {parser: parser_instance, _marker: PhantomData})
            },
            Err(error) => {
                let err_msg = format!("Failed to create Python parser, class='{}', error='{:?}'", class_name, error);
                Err(Error::verbatim_error(err_msg))
            }
        }
    }
}

impl<P: Pipe> PythonParser<P> {
    pub fn process_parsing<'p>(&mut self, py: Python<'p>, logmsg: PyLogMessage, message: &str) -> PyResult<PyObject> {
        debug!("Trying to call parse() method on Python parser");
        self.parser.call_method(py, "parse", (logmsg, message), None)
    }
    pub fn process_parse_result<'p>(py: Python<'p>, result: PyObject) -> PyResult<bool> {
        debug!("Trying to check the result of parse()");
        result.extract::<bool>(py)
    }
    pub fn call_parse<'p>(&mut self, py: Python<'p>, logmsg: PyLogMessage, input: &str) -> PyResult<bool> {
        let result = try!(self.process_parsing(py, logmsg, input));
        PythonParser::<P>::process_parse_result(py, result)
    }
}

impl<P: Pipe> Parser<P> for PythonParser<P> {
    fn parse(&mut self, _: &mut P, logmsg: &mut LogMessage, input: &str) -> bool {
        let gil = Python::acquire_gil();
        let py = gil.python();
        match PyLogMessage::new(py, logmsg.clone()) {
            Ok(pylogmsg) => {
                match self.call_parse(py, pylogmsg, input) {
                    Ok(result) => result,
                    Err(error) => {
                        error!("Failed to extract return value of parse() method: {:?}", error);
                        false
                    }
                }
            },
            // I didn't find a way to test this case :-(
            Err(error) => {
                error!("Failed to create PyLogMessage: {:?}", error);
                false
            }
        }
    }
}

parser_plugin!(PythonParserBuilder<LogParser>);
