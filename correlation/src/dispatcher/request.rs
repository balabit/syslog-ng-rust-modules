// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::sync::Arc;
use std::time::Duration;

use reactor;
use Event;


#[derive(Clone, Debug)]
pub enum Request<E: Event> {
    Message(Arc<E>),
    Timer(Duration),
    Exit,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RequestHandle {
    Message,
    Timer,
    Exit,
}

impl<E: Event> reactor::Event for Request<E> {
    type Handle = RequestHandle;
    fn handle(&self) -> Self::Handle {
        match *self {
            Request::Message(_) => RequestHandle::Message,
            Request::Timer(_) => RequestHandle::Timer,
            Request::Exit => RequestHandle::Exit,
        }
    }
}
