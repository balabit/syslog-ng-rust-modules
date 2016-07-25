extern crate python_parser;
extern crate cpython;
extern crate syslog_ng_common;

use cpython::Python;
use python_parser::PyLogMessage;
use syslog_ng_common::{LogMessage, SYSLOG_NG_INITIALIZED, syslog_ng_global_init};

#[test]
fn test_getitem() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe {
            syslog_ng_global_init();
        }
    });

    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut logmsg = LogMessage::new();
    logmsg.insert("a", "b".as_bytes());
    logmsg.insert("INVALID_UTF8", &b"hi \xFF"[..]);

    let pylogmsg = PyLogMessage::new(py, logmsg).unwrap();

    assert_eq!(pylogmsg.__getitem__(py, "a".to_string()).unwrap(),
               "b".to_string());
    assert_eq!(pylogmsg.__getitem__(py, "NON_EXISTING_KEY".to_string()).unwrap(),
               "".to_string());
    let _ = pylogmsg.__getitem__(py, "INVALID_UTF8".to_string()).err().unwrap();
}

#[test]
fn test_setitem() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe {
            syslog_ng_global_init();
        }
    });

    let gil = Python::acquire_gil();
    let py = gil.python();

    let logmsg = LogMessage::new();

    let pylogmsg = PyLogMessage::new(py, logmsg).unwrap();
    pylogmsg.__setitem__(py, "a".to_string(), "b".to_string()).ok().unwrap();

    assert_eq!(pylogmsg.__getitem__(py, "a".to_string()).unwrap(),
               "b".to_string());
}
