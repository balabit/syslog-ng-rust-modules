use std::sync::mpsc;
use std::thread;

use super::{Command, Event};

#[derive(Debug)]
pub struct TimerEvent(pub u32);

pub struct Timer;

impl Timer {
    pub fn from_chan(ms: u32, tx: mpsc::Sender<Command>) {
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
