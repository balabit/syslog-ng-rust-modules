use syslog_ng_sys::parser::RustParserProxy;
use syslog_ng_sys::{c_int, c_char, ssize_t};
use syslog_ng_sys::{from_c_str_to_owned_string, from_c_str_to_borrowed_str};
use syslog_ng_sys::LogMessage;
use syslog_ng_sys::RustParser;

use dummy_parser::DummyParser;

#[no_mangle]
pub extern fn rust_parser_proxy_init(this: &mut RustParserProxy) -> c_int {
    let res = this.parser.init();

    match res {
        true => 1,
        false => 0
    }
}

#[no_mangle]
pub extern fn rust_parser_proxy_free(_: Box<RustParserProxy>) {
}

#[no_mangle]
pub extern fn rust_parser_proxy_set_option(slf: &mut RustParserProxy, key: *const c_char, value: *const c_char) {
    let k = from_c_str_to_owned_string(key);
    let v = from_c_str_to_owned_string(value);

    slf.parser.set_option(k, v);
}

#[no_mangle]
pub extern fn rust_parser_proxy_process(this: &mut RustParserProxy, msg: &mut LogMessage, input: *const c_char, _: ssize_t) -> c_int {
    let input = from_c_str_to_borrowed_str(input);

    match this.parser.process(msg, input) {
        true => 0,
        false => 1
    }
}

#[no_mangle]
pub extern fn rust_parser_proxy_new(_: *const c_char) -> Box<RustParserProxy> {
    let parser = Box::new(DummyParser::new()) as Box<RustParser>;

    Box::new(RustParserProxy::new(parser))
}
