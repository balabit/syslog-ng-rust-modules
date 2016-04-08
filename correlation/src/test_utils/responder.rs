// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use Response;
use dispatcher::response::ResponseSender;
use Event;

#[derive(Clone)]
pub struct MockResponseSender<E: Event>(pub Vec<Response<E>>);

impl<E: Event> Default for MockResponseSender<E> {
    fn default() -> MockResponseSender<E> {
        MockResponseSender(Vec::default())
    }
}

impl<E: Event> ResponseSender<E> for MockResponseSender<E> {
    fn send_response(&mut self, response: Response<E>) {
        self.0.push(response);
    }
}
