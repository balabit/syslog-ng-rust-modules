use syslog_ng_sys::*;
use dummy_filter::DummyFilter;
use in_list_filter::InListFilter;

use std::ptr;
use std::mem;

#[no_mangle]
pub extern fn rust_filter_proxy_init(this: &mut RustFilterProxy, cfg: &GlobalConfig) {
    this.filter.init(cfg)
}

#[no_mangle]
pub extern fn rust_filter_proxy_eval(this: &mut RustFilterProxy, msg: &mut LogMessage) -> c_int {
    match this.filter.eval(msg) {
        true => 1,
        false => 0
    }
}

#[no_mangle]
pub extern fn rust_filter_proxy_free(_ : Box<RustFilterProxy>) {
}

#[no_mangle]
pub extern fn rust_filter_proxy_set_option(this: &mut RustFilterProxy, key: *const c_char, value: *const c_char) {
    let k = from_c_str_to_owned_string(key);
    let v = from_c_str_to_owned_string(value);

    this.filter.set_option(k, v);
}

#[no_mangle]
pub extern fn rust_filter_proxy_new(filter_name: *const c_char) -> Box<RustFilterProxy> {
    let filter = create_new_impl(filter_name);

    unsafe { 
        let result = match filter {
            Some(a) => a,
                // converts *mut to a Box
            None => {
                mem::transmute::<*mut RustFilterProxy, Box<RustFilterProxy>>(ptr::null_mut())
            }
        };

        return result
    }
}

fn create_new_impl(filter_name: *const c_char) -> Option<Box<RustFilterProxy>> {
    let name = from_c_str_to_borrowed_str(filter_name);

    let filter: Option<Box<RustFilter>> = match name {
        "dummy" => {
            Some(Box::new(DummyFilter::new()) as Box<RustFilter>)
        },
        "in-list" => {
            Some(Box::new(InListFilter::new()) as Box<RustFilter>)
        },
        _ => {
            msg_debug!("rust_filter_new(): {:?} not found, returning None", name);
            None
        },
    };

    match filter {
        Some(filter) => {
            Some(Box::new(RustFilterProxy{filter: filter}))
        },
        None => None
    }
}
