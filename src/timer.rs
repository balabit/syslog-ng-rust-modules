use std::sync::mpsc;
use std::thread;

use MiliSec;
use dispatcher::request::{ExternalRequest, Request};

#[derive(Clone, Copy, Debug)]
pub struct TimerEvent(pub MiliSec);

pub struct Timer;

impl Timer {
    pub fn from_chan(ms: MiliSec, tx: mpsc::Sender<ExternalRequest>) {
        thread::spawn(move || {
            loop {
                thread::sleep_ms(ms);
                if tx.send(Request::Timer(TimerEvent(ms))).is_err() {
                    break;
                }
            }
        });
    }
}
