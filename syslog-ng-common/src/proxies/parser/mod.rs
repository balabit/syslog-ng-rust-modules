use syslog_ng_sys::LogMessage;
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
    use $crate::sys::{c_int, c_char, ssize_t};
    use $crate::sys::{from_c_str_to_owned_string, from_c_str_to_borrowed_str};
    use $crate::sys::LogMessage;
    use $crate::sys::LogParser;
    use $crate::logger::init_logger;
    use $crate::proxies::parser::ParserProxy;

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
        let k = from_c_str_to_owned_string(key);
        let v = from_c_str_to_owned_string(value);

        slf.set_option(k, v);
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_process(this: &mut ParserProxy<$name>, msg: &mut LogMessage, input: *const c_char, _: ssize_t) -> c_int {
        let input = from_c_str_to_borrowed_str(input);

        match this.process(msg, input) {
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
