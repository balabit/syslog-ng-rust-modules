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
    use $crate::commit_suicide;

    use std::ffi::CStr;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use super::*;

    fn bool_to_int(result: bool) -> c_int {
        match result {
            true => 1,
            false => 0
        }
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_init(this: &mut ParserProxy<$name>) -> c_int {
        let mut wrapper = AssertUnwindSafe(this);

        match catch_unwind(move || wrapper.init()) {
            Ok(init_result) => bool_to_int(init_result),
            Err(error) => {
                error!("native_parser_proxy_init() panicked, but the panic was cought: {:?}", error);
                commit_suicide();
            }
        }
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_deinit(this: &mut ParserProxy<$name>) -> c_int {
        let result = this.deinit();
        bool_to_int(result)
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_free(proxy: Box<ParserProxy<$name>>) {
        let wrapper = AssertUnwindSafe(proxy);

        match catch_unwind(move || { let _ = wrapper; } ) {
            Err(error) => {
                error!("native_parser_proxy_free() panicked, but the panic was cought: {:?}", error);
                commit_suicide();
            },
            _ => (),
        }
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_set_option(this: &mut ParserProxy<$name>, key: *const c_char, value: *const c_char) {
        let mut wrapper_this = AssertUnwindSafe(this);
        let wrapper_key = AssertUnwindSafe(key);
        let wrapper_value = AssertUnwindSafe(value);

        let result = catch_unwind(move || {
            let k: String = unsafe { CStr::from_ptr(*wrapper_key).to_owned().to_string_lossy().into_owned() };
            let v: String = unsafe { CStr::from_ptr(*wrapper_value).to_owned().to_string_lossy().into_owned() };

            wrapper_this.set_option(k, v);
        });

        if let Err(caught_error) = result {
            error!("native_parser_proxy_set_option() panicked, but the panic was cought: {:?}", caught_error);
            commit_suicide();
        }
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
    pub extern fn native_parser_proxy_new(cfg: *mut $crate::sys::GlobalConfig) -> *mut ParserProxy<$name> {
        let result = catch_unwind(move || {
            init_logger();
            let cfg = GlobalConfig::borrow(cfg);
            Box::new(ParserProxy::new(cfg))
        });

        match result {
            Ok(proxy) => Box::into_raw(proxy),
            Err(error) => {
                error!("native_parser_proxy_new() panicked, but the panic was cought: {:?}", error);
                commit_suicide();
            }
        }
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_clone(slf: &ParserProxy<$name>) -> Box<ParserProxy<$name>> {
        let cloned = (*slf).clone();
        Box::new(cloned)
    }
}
    }
}
