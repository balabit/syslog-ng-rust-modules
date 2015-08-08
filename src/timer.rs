use std::sync::mpsc;
use std::thread;

use super::{Command, Event, MiliSec};

#[derive(Debug)]
pub struct TimerEvent(pub MiliSec);

pub struct Timer;

impl Timer {
    pub fn from_chan(ms: MiliSec, tx: mpsc::Sender<Command>) {
        thread::spawn(move || {
            loop {
                thread::sleep_ms(ms);
                if tx.send(Command::Dispatch(Event::Timer(TimerEvent(ms)))).is_err() {
                    break;
                }
            }
        });
    }
}
