// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use dispatcher::ResponseHandle;
use Response;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc;
use dispatcher::request::Request;

use reactor::EventHandler;
use action::Alert;

pub struct MessageEventHandler {
    pub responses: Rc<RefCell<Vec<Alert>>>,
}

impl EventHandler<Response, mpsc::Sender<Request>> for MessageEventHandler {
    fn handle_event(&mut self, event: Response, _: &mut mpsc::Sender<Request>) {
        if let Response::Alert(event) = event {
            self.responses.borrow_mut().push(event);
        }
    }
    fn handle(&self) -> ResponseHandle {
        ResponseHandle::Alert
    }
}
