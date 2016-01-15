use LogMessage;
use syslog_ng_sys::LogParser;

mod option_error;
mod proxy;

pub use self::option_error::OptionError;
pub use self::proxy::ParserProxy;

pub trait ParserBuilder: Clone {
    type Parser: Parser;
    fn new() -> Self;
    fn option(&mut self, name: String, value: String);
    fn parent(&mut self, _: *mut LogParser) {}
    fn build(self) -> Result<Self::Parser, OptionError>;
}

pub trait Parser: Clone {
    fn parse(&mut self, msg: &mut LogMessage, input: &str) -> bool;
}

#[macro_export]
macro_rules! parser_plugin {
    ($name:ty) => {

pub mod _parser_plugin {
    use $crate::{c_int, c_char, ssize_t};
    use $crate::LogMessage;
    use $crate::LogParser;
    use $crate::logger::init_logger;
    use $crate::proxies::parser::ParserProxy;

    use std::ffi::CStr;

    use super::*;

    #[no_mangle]
    pub extern fn native_parser_proxy_init(this: &mut ParserProxy<$name>) -> c_int {
        let res = this.init();

        match res {
            true => 1,
            false => 0
        }
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_free(_: Box<ParserProxy<$name>>) {
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_set_option(slf: &mut ParserProxy<$name>, key: *const c_char, value: *const c_char) {
        let k: String = unsafe { CStr::from_ptr(key).to_owned().to_string_lossy().into_owned() };
        let v: String = unsafe { CStr::from_ptr(value).to_owned().to_string_lossy().into_owned() };

        slf.set_option(k, v);
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_process(this: &mut ParserProxy<$name>, msg: *mut $crate::sys::LogMessage, input: *const c_char, _: ssize_t) -> c_int {
        let input = unsafe { CStr::from_ptr(input).to_str() };
        let mut msg = LogMessage::wrap_raw(msg);
        let result = match input {
            Ok(input) => this.process(&mut msg, input),
            Err(err) => {
                error!("{}", err);
                false
            }
        };

        match result {
            true => 1,
            false => 0
        }
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_new(parent: *mut LogParser) -> Box<ParserProxy<$name>> {
        init_logger();
        let mut proxy = ParserProxy::new();
        proxy.parent(parent);
        Box::new(proxy)
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_clone(slf: &ParserProxy<$name>) -> Box<ParserProxy<$name>> {
        let cloned = (*slf).clone();
        Box::new(cloned)
    }
}
    }
}
