use syslog_ng_sys::LogMessage;
use syslog_ng_sys::LogParser;

#[repr(C)]
#[derive(Clone)]
pub struct RustParserProxy<P> where P: RustParser {
    pub parser: Option<P>,
    pub builder: Option<P::Builder>
}

impl<P> RustParserProxy<P> where P: RustParser {
    pub fn new() -> RustParserProxy<P> {
        RustParserProxy {
            parser: None,
            builder: Some(P::Builder::new())
        }
    }

    pub fn init(&mut self) -> bool {
        let builder = self.builder.take().expect("Called init when builder was not set");
        match builder.build() {
            Ok(parser) => {
                self.parser = Some(parser);
                self.parser.as_mut().expect("Called init on a non-existing Rust Parser").init()
            },
            Err(error) => {
                error!("Error: {:?}", error);
                false
            }
        }
    }

    pub fn set_option(&mut self, name: String, value: String) {
        if self.builder.is_none() {
            self.builder = Some(P::Builder::new());
        }

        let builder = self.builder.as_mut().expect("Failed to get builder on a RustParserProxy");
        builder.option(name, value);
    }

    pub fn process(&mut self, msg: &mut LogMessage, input: &str) -> bool {
        self.parser.as_mut().expect("Called process on a non-existing Rust parser").process(msg, input)
    }

    pub fn parent(&mut self, parent: *mut LogParser) {
        let builder = self.builder.as_mut().expect("Failed to get a builder on a new parser proxy instance");
        builder.parent(parent);
    }
}

#[derive(Debug)]
pub enum OptionError {
    MissingRequiredOption(String),
    InvalidValue{value: String, expected_value: String}
}

pub trait RustParserBuilder: Clone {
    type Parser: RustParser;
    fn new() -> Self;
    fn option(&mut self, name: String, value: String);
    fn parent(&mut self, _: *mut LogParser) {}
    fn build(self) -> Result<Self::Parser, OptionError>;
}

pub trait RustParser: Clone {
    type Builder: RustParserBuilder<Parser=Self>;
    fn init(&mut self) -> bool { true }
    fn process(&mut self, msg: &mut LogMessage, input: &str) -> bool;
}

#[macro_export]
macro_rules! parser_plugin {
    ($name:ty) => {

pub mod _parser_plugin {
    extern crate syslog_ng_sys;
    use $crate::sys::{c_int, c_char, ssize_t};
    use $crate::sys::{from_c_str_to_owned_string, from_c_str_to_borrowed_str};
    use $crate::sys::LogMessage;
    use $crate::sys::LogParser;
    use $crate::logger::init_logger;
    use $crate::proxies::parser::{
        RustParser,
        RustParserProxy,
    };

    use super::*;

    #[no_mangle]
    #[allow(dead_code)]
    pub extern fn rust_parser_proxy_init(this: &mut RustParserProxy<$name>) -> c_int {
        let res = this.init();

        match res {
            true => 1,
            false => 0
        }
    }

    #[no_mangle]
    #[allow(dead_code)]
    pub extern fn rust_parser_proxy_free(_: Box<RustParserProxy<$name>>) {
    }

    #[no_mangle]
    #[allow(dead_code)]
    pub extern fn rust_parser_proxy_set_option(slf: &mut RustParserProxy<$name>, key: *const c_char, value: *const c_char) {
        let k = from_c_str_to_owned_string(key);
        let v = from_c_str_to_owned_string(value);

        slf.set_option(k, v);
    }

    #[no_mangle]
    #[allow(dead_code)]
    pub extern fn rust_parser_proxy_process(this: &mut RustParserProxy<$name>, msg: &mut LogMessage, input: *const c_char, _: ssize_t) -> c_int {
        let input = from_c_str_to_borrowed_str(input);

        match this.process(msg, input) {
            true => 1,
            false => 0
        }
    }

    #[no_mangle]
    #[allow(dead_code)]
    pub extern fn rust_parser_proxy_new(parent: *mut LogParser) -> Box<RustParserProxy<$name>> {
        init_logger();
        let mut proxy = RustParserProxy::new();
        proxy.parent(parent);
        Box::new(proxy)
    }

    #[no_mangle]
    #[allow(dead_code)]
    pub extern fn rust_parser_proxy_clone(slf: &RustParserProxy<$name>) -> Box<RustParserProxy<$name>> {
        let cloned = (*slf).clone();
        Box::new(cloned)
    }
}
    }
}
