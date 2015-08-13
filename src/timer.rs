use std::sync::mpsc;
use std::thread;

use super::{Event, MiliSec, Request};

#[derive(Debug)]
pub struct TimerEvent(pub MiliSec);

pub struct Timer;

impl Timer {
    pub fn from_chan(ms: MiliSec, tx: mpsc::Sender<Request>) {
        thread::spawn(move || {
            loop {
                thread::sleep_ms(ms);
                if tx.send(Request::Dispatch(Event::Timer(TimerEvent(ms)))).is_err() {
                    break;
                }
            }
        });
    }
}
