// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use Alert;
use reactor::Event;
use Event as MsgEvent;

pub mod handlers;
pub mod request;

#[derive(Debug, Clone)]
pub enum Response<E: MsgEvent> {
    Exit,
    Alert(Alert<E>),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ResponseHandle {
    Exit,
    Alert,
}

impl<E: MsgEvent> Event for Response<E> {
    type Handle = ResponseHandle;
    fn handle(&self) -> Self::Handle {
        match *self {
            Response::Exit => ResponseHandle::Exit,
            Response::Alert(_) => ResponseHandle::Alert,
        }
    }
}
