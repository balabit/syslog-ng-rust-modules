// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use state::State;
use dispatcher::response::ResponseSender;
use context::base::BaseContext;
use Event;
use Template;

pub use config::action::message::Alert;

pub trait Action<E, T> where E: Event, T: Template<Event=E> {
    fn on_opened(&self, state: &State<E>, context: &BaseContext<E, T>, &mut ResponseSender<E>);
    fn on_closed(&self, state: &State<E>, context: &BaseContext<E, T>, &mut ResponseSender<E>);
}
