// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::BTreeMap;

use context::ContextMap;
use dispatcher::demux::Demultiplexer;
use dispatcher::request::{RequestHandle, Request};
use reactor::{Event, EventDemultiplexer, EventHandler, Reactor, SharedData};
use dispatcher::response::ResponseSender;
use Event as MsgEvent;
use Template;

#[allow(type_complexity)]
pub struct RequestReactor<E, T> where E: MsgEvent, T: Template<Event=E> {
    handlers: BTreeMap<RequestHandle, Box<for<'a> EventHandler<Request<E>, SharedData<'a, E, T>>>>,
    demultiplexer: Demultiplexer<Request<E>>,
    pub context_map: ContextMap<E, T>,
    responder: Box<ResponseSender<E>>,
}

impl<E, T> RequestReactor<E, T> where E: MsgEvent, T: Template<Event=E> {
    pub fn new(demultiplexer: Demultiplexer<Request<E>>,
               context_map: ContextMap<E, T>,
               responder: Box<ResponseSender<E>>)
               -> RequestReactor<E, T> {
        RequestReactor {
            demultiplexer: demultiplexer,
            context_map: context_map,
            handlers: BTreeMap::new(),
            responder: responder,
        }
    }
}

impl<E, T> Reactor<E, T> for RequestReactor<E, T> where E: MsgEvent, T: Template<Event=E> {
    type Event = Request<E>;
    fn handle_events(&mut self) {
        let mut shared_data = SharedData::new(&mut self.context_map, &mut *self.responder);
        while let Some(request) = self.demultiplexer.select() {
            trace!("RequestReactor: got event");
            if let Some(handler) = self.handlers.get_mut(&request.handle()) {
                handler.handle_event(request, &mut shared_data);
            } else {
                trace!("RequestReactor: no handler found for event");
            }
        }
    }
    fn register_handler(&mut self,
                        handler: Box<for<'a> EventHandler<Self::Event, SharedData<'a, E, T>>>) {
        self.handlers.insert(handler.handle(), handler);
    }
    fn remove_handler_by_handle(&mut self, handler: &RequestHandle) {
        self.handlers.remove(handler);
    }
}
