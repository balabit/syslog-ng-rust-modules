// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use context::ContextMap;
use Event as MsgEvent;
use Template;
use Alert;

use std::collections::VecDeque;

pub struct SharedData<'a, E, T> where E: 'a + MsgEvent, T: 'a + Template<Event=E> {
    pub responder: &'a mut VecDeque<Alert<E>>,
    pub map: &'a mut ContextMap<E, T>,
}

impl<'a, E, T> SharedData<'a, E, T> where E: 'a + MsgEvent, T: Template<Event=E> {
    pub fn new(map: &'a mut ContextMap<E, T>, responder: &'a mut VecDeque<Alert<E>>) -> SharedData<'a, E, T> {
        SharedData {
            map: map,
            responder: responder,
        }
    }
}

pub trait EventHandler<T, D> {
    fn handle_event(&mut self, event: T, shared_data: &mut D);
}
