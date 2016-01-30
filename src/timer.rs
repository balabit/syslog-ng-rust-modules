use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use dispatcher::request::{ExternalRequest, Request};

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
    pub fn from_chan(duration: Duration, tx: mpsc::Sender<ExternalRequest>) {
        thread::spawn(move || {
            while let Ok(_) = tx.send(Request::Timer(TimerEvent(duration))) {
                thread::sleep(duration);
            }
        });
    }
}
