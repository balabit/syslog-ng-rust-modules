// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use Event;

use dispatcher::request::Request;

#[derive(Clone, Copy, Debug)]
pub struct TimerEvent(pub Duration);

impl TimerEvent {
    #[allow(dead_code)]
    pub fn from_millis(ms: u64) -> TimerEvent {
        TimerEvent(Duration::from_millis(ms))
    }
}

pub struct Timer;

impl Timer {
    pub fn from_chan<E: 'static + Event>(duration: Duration, tx: mpsc::Sender<Request<E>>) {
        thread::spawn(move || {
            while let Ok(_) = tx.send(Request::Timer(TimerEvent(duration))) {
                thread::sleep(duration);
            }
        });
    }
}
