use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use dispatcher::request::{ExternalRequest, Request};

#[derive(Clone, Copy, Debug)]
pub struct TimerEvent(pub Duration);

pub struct Timer;

impl Timer {
    pub fn from_chan(duration: Duration, tx: mpsc::Sender<ExternalRequest>) {
        thread::spawn(move || {
            loop {
                thread::sleep(duration);
                if tx.send(Request::Timer(TimerEvent(duration))).is_err() {
                    break;
                }
            }
        });
    }
}
