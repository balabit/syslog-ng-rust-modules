// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use dispatcher::response::ResponseSender;
use context::ContextMap;
use Event as MsgEvent;
use Template;

pub struct SharedData<'a, E, T> where E: 'a + MsgEvent, T: 'a + Template<Event=E> {
    pub responder: &'a mut ResponseSender<E>,
    pub map: &'a mut ContextMap<E, T>,
}

impl<'a, E, T> SharedData<'a, E, T> where E: 'a + MsgEvent, T: Template<Event=E> {
    pub fn new(map: &'a mut ContextMap<E, T>, responder: &'a mut ResponseSender<E>) -> SharedData<'a, E, T> {
        SharedData {
            map: map,
            responder: responder,
        }
    }
}

pub trait EventHandler<T: Event, D> {
    fn handle_event(&mut self, event: T, shared_data: &mut D);
    fn handle(&self) -> T::Handle;
}

pub trait EventDemultiplexer {
    type Event: Event;
    fn select(&mut self) -> Option<Self::Event>;
}

pub trait Reactor<E, T> where E: MsgEvent, T: Template<Event=E> {
    type Event: Event;
    fn handle_events(&mut self);
    fn register_handler(&mut self,
                        handler: Box<for<'a> EventHandler<Self::Event, SharedData<'a, E, T>>>);
    fn remove_handler_by_handle(&mut self,
                                 handler: &<<Self as Reactor<E, T>>::Event as Event>::Handle);
}

pub trait Event {
    type Handle;
    fn handle(&self) -> Self::Handle;
}
