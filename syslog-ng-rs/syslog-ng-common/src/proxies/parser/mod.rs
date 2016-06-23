// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use LogMessage;
use Pipe;

mod option_error;
mod proxy;

pub use self::option_error::OptionError;
pub use self::proxy::ParserProxy;
use GlobalConfig;

pub trait ParserBuilder<P: Pipe>: Clone {
    type Parser: Parser<P>;
    fn new(GlobalConfig) -> Self;
    fn option(&mut self, _name: String, _value: String) {}
    fn build(self) -> Result<Self::Parser, OptionError>;
}

pub trait Parser<P: Pipe> {
    fn init(&mut self) -> bool { true }
    fn deinit(&mut self) -> bool { true }
    fn parse(&mut self, pipe: &mut P, msg: &mut LogMessage, input: &str) -> bool;
}

#[macro_export]
macro_rules! parser_plugin {
    ($name:ty) => {

pub mod _parser_plugin {
    use $crate::{c_int, c_char};
    use $crate::LogMessage;
    use $crate::LogParser;
    use $crate::init_logger;
    use $crate::ParserProxy;
    use $crate::GlobalConfig;

    use std::ffi::CStr;

    use super::*;

    fn bool_to_int(result: bool) -> c_int {
        match result {
            true => 1,
            false => 0
        }
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_init(this: &mut ParserProxy<$name>) -> c_int {
        let result = this.init();
        bool_to_int(result)
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
    pub extern fn native_parser_proxy_process(this: &mut ParserProxy<$name>, parent: *mut $crate::sys::LogParser, msg: *mut $crate::sys::LogMessage, input: *const c_char) -> c_int {
        let input = unsafe { CStr::from_ptr(input).to_str() };
        let mut parent = LogParser::wrap_raw(parent);
        let mut msg = LogMessage::wrap_raw(msg);
        let result = match input {
            Ok(input) => this.process(&mut parent, &mut msg, input),
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
    pub extern fn native_parser_proxy_new(cfg: *mut $crate::sys::GlobalConfig) -> Box<ParserProxy<$name>> {
        init_logger();
        let cfg = GlobalConfig::borrow(cfg);
        Box::new(ParserProxy::new(cfg))
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_clone(slf: &ParserProxy<$name>) -> Box<ParserProxy<$name>> {
        let cloned = (*slf).clone();
        Box::new(cloned)
    }
}
    }
}
