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
use Event;
use Template;

#[derive(Default)]
pub struct MessageEventHandler;

impl<'a, E, T> EventHandler<Request<E>, SharedData<'a, E, T>> for MessageEventHandler where E: 'a + Event, T: Template<Event=E> {
    fn handle_event(&mut self, event: Request<E>, data: &mut SharedData<E, T>) {
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
