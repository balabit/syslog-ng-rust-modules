// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use LogMessage;
use Pipe;

use std::panic::{UnwindSafe, catch_unwind};

mod error;
mod proxy;

pub use self::error::Error;
pub use self::proxy::ParserProxy;
use GlobalConfig;
use commit_suicide;
use c_int;

/// Trait that can build a particular `Parser` instance.
///
/// The initialization of a parser plugin is split into two halves. First, a `ParserBuilder` instance
/// is created with it's `new()` method, then several configuration options can be set with
/// the `option()` method.
///
/// The parser instance is created when `build()` is called. It returns either a parser instance,
/// or an error indicating why the building wasn't successful.
pub trait ParserBuilder<P: Pipe>: Clone {
    /// Parser is the type that this builder implementation is able to build.
    type Parser: Parser<P>;
    /// Creates a new `ParserBuilder` instance.
    fn new(GlobalConfig) -> Self;
    /// Sets a configuration option.
    fn option(&mut self, _name: String, _value: String) -> Result<(), Error> { Ok(()) }
    /// Tries to build a parser instance.
    fn build(self) -> Result<Self::Parser, Error>;
}

/// The `Parser` trait is used to represent a `parser` according to syslog-ng's terminology.
pub trait Parser<P: Pipe> {
    /// `init()` is called to indicate that the parser should be ready for log message processing.
    ///
    /// It is mainly used in conjunction with `deinit()`: the two methods shold be symmetrical. This
    /// means, that if `init()` starts a timer thread, `deinit()` should stop it.
    fn init(&mut self) -> bool { true }
    /// `deinit()` is called to indicate that the parser is temporarily suspended (or before dropping
    /// it permanently).
    fn deinit(&mut self) -> bool { true }
    /// Parses `input` and inserts the new key-value pairs into `msg`. `pipe` represents the parent
    /// `LogPipe`. It can be mocked out to simplify the testing without syslog-ng.
    fn parse(&mut self, pipe: &mut P, msg: &mut LogMessage, input: &str) -> bool;
}

/// Converts a `bool` to a `c_int`
pub fn bool_to_int(result: bool) -> c_int {
    match result {
        true => 1,
        false => 0
    }
}

/// Calls `unwind_safe_call` and returns the result if it was successul. If a panic occured,
/// an error message is logged and the current process is aborted.
pub fn abort_on_panic<F, R>(func_name_suffix: &str, unwind_safe_call: F) -> R
where F: UnwindSafe + FnOnce() -> R {
    match catch_unwind(unwind_safe_call) {
        Ok(result) => result,
        Err(error) => {
            error!("native_parser_proxy_{}() panicked, but the panic was caught: {:?}", func_name_suffix,  error);
            commit_suicide();
        }
    }
}

/// This macro generates FFI bindings for a parser.
///
/// It takes a `ParserBuilder` as its parameter and generates several functions. Only one `parser_plugin!`
/// can be used in a compilation unit.
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
    use $crate::abort_on_panic;
    use $crate::bool_to_int;

    use std::ffi::CStr;
    use std::panic::AssertUnwindSafe;

    use super::*;

    #[no_mangle]
    pub extern fn native_parser_proxy_init(this: &mut ParserProxy<$name>) -> c_int {
        let mut wrapper = AssertUnwindSafe(this);

        let unwind_safe_call = move || {
            bool_to_int(wrapper.init())
        };

        abort_on_panic("init", unwind_safe_call)
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_deinit(this: &mut ParserProxy<$name>) -> c_int {
        let mut wrapper = AssertUnwindSafe(this);

        let unwind_safe_call = move || {
            bool_to_int(wrapper.deinit())
        };

        abort_on_panic("deinit", unwind_safe_call)
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_free(proxy: *mut ParserProxy<$name>) {
        let wrapper = AssertUnwindSafe(proxy);

        let unwind_safe_call = move || {
            let _ = unsafe { Box::from_raw(*wrapper) };
        };

        abort_on_panic("free", unwind_safe_call)
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_set_option(this: &mut ParserProxy<$name>, key: *const c_char, value: *const c_char) -> c_int {
        let mut wrapper_this = AssertUnwindSafe(this);
        let wrapper_key = AssertUnwindSafe(key);
        let wrapper_value = AssertUnwindSafe(value);

        let unwind_safe_call = move || {
            let k: String = unsafe { CStr::from_ptr(*wrapper_key).to_owned().to_string_lossy().into_owned() };
            let v: String = unsafe { CStr::from_ptr(*wrapper_value).to_owned().to_string_lossy().into_owned() };

            bool_to_int(wrapper_this.set_option(k, v))
        };

        abort_on_panic("set_option", unwind_safe_call)
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_process(this: &mut ParserProxy<$name>, parent: *mut $crate::sys::LogParser, msg: *mut $crate::sys::LogMessage, input: *const c_char) -> c_int {
        let mut wrapper_this = AssertUnwindSafe(this);
        let wrapper_parent = AssertUnwindSafe(parent);
        let wrapper_msg = AssertUnwindSafe(msg);
        let wrapper_input = AssertUnwindSafe(input);

        let unwind_safe_call = move || {
            let input = unsafe { CStr::from_ptr(*wrapper_input).to_str() };

            let result: bool = match input {
                Ok(input) => {
                    let mut parent = LogParser::wrap_raw(*wrapper_parent);
                    let mut msg = LogMessage::wrap_raw(*wrapper_msg);

                    wrapper_this.process(&mut parent, &mut msg, input)
                },
                Err(err) => {
                    error!("{}", err);
                    false
                }
            };

            bool_to_int(result)
        };

        abort_on_panic("process", unwind_safe_call)
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_new(cfg: *mut $crate::sys::GlobalConfig) -> *mut ParserProxy<$name> {
        let unwind_safe_call = move || {
            init_logger();
            let cfg = GlobalConfig::borrow(cfg);
            let proxy = Box::new(ParserProxy::new(cfg));
            Box::into_raw(proxy)
        };

        abort_on_panic("new", unwind_safe_call)
    }

    #[no_mangle]
    pub extern fn native_parser_proxy_clone(this: &ParserProxy<$name>) -> *mut ParserProxy<$name> {
        let wrapper_this = AssertUnwindSafe(this);

        let unwind_safe_call = move || {
            let cloned = (*wrapper_this).clone();
            Box::into_raw(Box::new(cloned))
        };

        abort_on_panic("clone", unwind_safe_call)
    }
}
    }
}
