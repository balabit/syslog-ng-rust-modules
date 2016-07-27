use syslog_ng_common::LogMessage;

use std::cell::RefCell;
use cpython::{Python, PyResult, PyErr};
use cpython::exc::UnicodeDecodeError;

py_class!(class PyLogMessage |py| {
    data logmsg: RefCell<LogMessage>;

    def __getitem__(&self, key: String) -> PyResult<String> {
        let key: &str = key.as_ref();
        let logmsg = self.logmsg(py).borrow();
        if let Some(value) = logmsg.get(key) {
            match ::std::str::from_utf8(value) {
                Ok(value) => Ok(value.to_string()),
                Err(err) => {
                    let u = try!(UnicodeDecodeError::new_utf8(py, value, err));
                    Err(PyErr::from_instance(py, u))
                }
            }
        } else {
            Ok("".to_string())
        }
    }

    def __setitem__(&self, key: String, value: String) -> PyResult<()> {
        let key: &str = key.as_ref();
        let mut msg = self.logmsg(py).borrow_mut();
        msg.insert(key, value.as_bytes());
        Ok(())
    }
});

impl PyLogMessage {
    pub fn new(py: Python, logmsg: LogMessage) -> PyResult<PyLogMessage> {
        PyLogMessage::create_instance(py, RefCell::new(logmsg))
    }
}
