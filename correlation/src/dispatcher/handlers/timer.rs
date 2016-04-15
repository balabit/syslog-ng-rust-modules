// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use dispatcher::request::{Request, RequestHandle};
use reactor::{EventHandler, SharedData};
use Event;
use Template;

#[derive(Default)]
pub struct TimerEventHandler;

impl<'a, E, T> EventHandler<Request<E>, SharedData<'a, E, T>> for TimerEventHandler where E: 'a + Event, T: Template<Event=E>{
    fn handle_event(&mut self, event: Request<E>, data: &mut SharedData<E, T>) {
        for i in data.map.contexts_mut() {
            i.on_event(event.clone(), data.responder);
        }
    }
    fn handle(&self) -> RequestHandle {
        RequestHandle::Timer
    }
}
