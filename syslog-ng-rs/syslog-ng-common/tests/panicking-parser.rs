// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate syslog_ng_common;
#[macro_use]
extern crate log;
extern crate libc;

use std::marker::PhantomData;
use std::ffi::CString;

use libc::{SIGABRT, waitpid, fork, WIFSIGNALED, WTERMSIG, pid_t};

use syslog_ng_common::{SYSLOG_NG_INITIALIZED, syslog_ng_global_init, ParserProxy, LogMessage,
                       Parser, ParserBuilder, OptionError, Pipe, GlobalConfig};
use syslog_ng_common::sys;

pub struct PanickingParser<P: Pipe>(PhantomData<P>);

pub struct PanickingParserBuilder<P: Pipe>(PhantomData<P>);

impl<P: Pipe> Drop for PanickingParser<P> {
    fn drop(&mut self) {
        panic!("panic! in Drop");
    }
}

impl<P: Pipe> ParserBuilder<P> for PanickingParserBuilder<P> {
    type Parser = PanickingParser<P>;
    fn new(_: GlobalConfig) -> Self {
        panic!("new() panicked");
    }
    fn option(&mut self, _: String, _: String) {
        panic!("option() panicked");
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        panic!("build() panicked");
    }
}

impl<P: Pipe> Parser<P> for PanickingParser<P> {
    fn parse(&mut self, _: &mut P, _: &mut LogMessage, _: &str) -> bool {
        panic!("parse() panicked");
    }

    fn deinit(&mut self) -> bool {
        panic!("deinit() panicked");
    }
}

impl<P: Pipe> Clone for PanickingParserBuilder<P> {
    fn clone(&self) -> Self {
        panic!("clone() panicked")
    }
}

// this verifies that the macro can be expanded
parser_plugin!(PanickingParserBuilder<LogParser>);

use _parser_plugin::{native_parser_proxy_new, native_parser_proxy_free,
                     native_parser_proxy_set_option, native_parser_proxy_init,
                     native_parser_proxy_deinit, native_parser_proxy_process,
                     native_parser_proxy_clone};


fn set_up_test() {
    SYSLOG_NG_INITIALIZED.call_once(|| {
        unsafe {
            syslog_ng_global_init();
        }
    });
}

pub fn fork_with_callbacks<C, P>(child_callback: C, parent_callback: P) -> Result<(), ()>
    where C: FnOnce(),
          P: FnOnce(pid_t)
{
    match unsafe { fork() } {
        0 => {
            child_callback();
            Ok(())
        }
        x if x > 0 => {
            parent_callback(x);
            Ok(())
        }
        -1 | _ => Err(()),
    }
}

fn assert_child_commits_suicide<C>(child_callback: C)
    where C: FnOnce()
{
    let parent_callback = |child_pid| {
        let mut status = 0;
        let options = 0;
        unsafe {
            let result = waitpid(child_pid, &mut status, options);
            assert!(result != -1);
            assert!(WIFSIGNALED(status));
            assert_eq!(SIGABRT, WTERMSIG(status));
        };
    };

    fork_with_callbacks(child_callback, parent_callback).unwrap();
}

#[test]
fn test_native_parser_proxy_new_wont_panic_even_if_the_proxy_panics() {
    assert_child_commits_suicide(|| {
        set_up_test();

        let cfg = GlobalConfig::new(0x0308);
        let _ = native_parser_proxy_new(cfg.raw_ptr());
    });
}

#[test]
fn test_native_parser_proxy_free_wont_panic_even_if_the_proxy_panics() {
    assert_child_commits_suicide(|| {
        set_up_test();


        let proxy = ParserProxy::with_builder_and_parser(None, Some(PanickingParser(PhantomData)));
        let _ = native_parser_proxy_free(Box::into_raw(Box::new(proxy)));
    });
}

#[test]
fn test_native_parser_proxy_set_option_wont_panic_even_if_the_proxy_panics() {
    assert_child_commits_suicide(|| {
        set_up_test();

        let mut proxy =
            ParserProxy::with_parser_and_builder(Some(PanickingParserBuilder(PhantomData)), None);
        let key = CString::new("key").unwrap();
        let value = CString::new("value").unwrap();
        let _ = native_parser_proxy_set_option(&mut proxy, key.as_ptr(), value.as_ptr());
    });
}

#[test]
fn test_native_parser_proxy_init_wont_panic_even_if_the_proxy_panics() {
    assert_child_commits_suicide(|| {
        set_up_test();

        let mut proxy =
            ParserProxy::with_parser_and_builder(Some(PanickingParserBuilder(PhantomData)), None);
        let _ = native_parser_proxy_init(&mut proxy);
    });
}

#[test]
fn test_native_parser_proxy_deinit_wont_panic_even_if_the_proxy_panics() {
    assert_child_commits_suicide(|| {
        set_up_test();

        let mut proxy = ParserProxy::with_parser_and_builder(None,
                                                             Some(PanickingParser(PhantomData)));
        let _ = native_parser_proxy_deinit(&mut proxy);
    });
}

#[test]
fn test_native_parser_proxy_process_wont_panic_even_if_the_proxy_panics() {
    assert_child_commits_suicide(|| {
        set_up_test();

        let mut proxy = ParserProxy::with_parser_and_builder(None,
                                                             Some(PanickingParser(PhantomData)));
        let parser: *mut sys::LogParser = ::std::ptr::null_mut();
        let input = CString::new("input").unwrap();
        let msg = LogMessage::new();
        let _ = native_parser_proxy_process(&mut proxy, parser, msg.0, input.as_ptr());
    });
}

#[test]
fn test_native_parser_proxy_clone_wont_panic_even_if_the_proxy_panics() {
    assert_child_commits_suicide(|| {
        set_up_test();

        let proxy = ParserProxy::with_parser_and_builder(Some(PanickingParserBuilder(PhantomData)),
                                                         None);
        let _ = native_parser_proxy_clone(&proxy);
    });
}
