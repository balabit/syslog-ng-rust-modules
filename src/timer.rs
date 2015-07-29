use std::thread;
use std::sync::mpsc;

pub struct Timer;

impl Timer {
    pub fn from_chan(ms: u32, tx: mpsc::SyncSender<()>) {
        thread::spawn(move || {
            loop {
                thread::sleep_ms(ms);
                if tx.send(()).is_err() {
                    break;
                }
            }
        });
    }
}
