// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use dispatcher::request::{Request, RequestHandle};
use context::context_map::StreamingIterator;
use reactor::{EventHandler, SharedData};

pub struct MessageEventHandler;

impl MessageEventHandler {
    pub fn new() -> MessageEventHandler {
        MessageEventHandler
    }
}

impl<'a> EventHandler<Request, SharedData<'a>> for MessageEventHandler {
    fn handle_event(&mut self, event: Request, data: &mut SharedData) {
        trace!("MessageEventHandler: handle_event()");
        if let Request::Message(event) = event {
            for i in event.ids() {
                let mut iter = data.map.contexts_iter_mut(i);
                while let Some(context) = iter.next() {
                    context.on_event(Request::Message(event.clone()), data.responder);
                }
            }
        } else {
            unreachable!("MessageEventHandler should only handle Message events");
        }
    }
    fn handle(&self) -> RequestHandle {
        RequestHandle::Message
    }
}
