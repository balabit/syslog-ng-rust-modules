use syslog_ng_common::LogMessage;

use cpython::{Python, ToPyObject, NoArgs,PyObject, PyResult, PyString};
use cpython::rustobject::{TypeBuilder, PyRustObject};
use cpython::ObjectProtocol; //for call method

fn getitem(py: Python, slf: &PyRustObject<LogMessage>, arg: &str) -> PyResult<PyString> {
    if let Some(value) = slf.get(py).get(arg) {
        let value = String::from_utf8_lossy(value);
        Ok(PyString::new(py, &value))
    } else {
        Ok(PyString::new(py, ""))
    }
}

fn setitem(py: Python, slf: &PyRustObject<LogMessage>, key: &str, value: &str) -> PyResult<NoArgs> {
    let msg = slf.get_mut(py);
    msg.insert(key, value.as_bytes());
    Ok(NoArgs)
}

pub struct PyLogMessage(PyRustObject<LogMessage>);

impl PyLogMessage {
    pub fn new<'p>(py: Python<'p>, logmsg: LogMessage) -> PyResult<PyLogMessage> {
        let mut b = TypeBuilder::<LogMessage>::new(py, "PyLogMessage");
        b.add("__getitem__", py_method!(getitem(arg: &str)));
        b.add("__setitem__", py_method!(setitem(key: &str, value: &str)));
        trace!("Trying to finish construction PyLogMessage");
        let built_type = try!(b.finish());
        let instance = built_type.create_instance(py, logmsg, ());
        Ok(PyLogMessage(instance))
    }
}

impl ToPyObject for PyLogMessage {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        self.0.to_py_object(py)
    }
    fn into_py_object(self, _py: Python) -> PyObject {
        self.0.into_py_object(_py)
    }
}
