use std::sync::mpsc::{Sender, channel, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use Timer;
use correlation::correlator::Correlator;
use correlation::{Event, Template};
use std::thread::{self, JoinHandle};

struct StopEvent;

pub struct Watchdog {
    sender: Sender<StopEvent>,
    _join_handle: JoinHandle<()>
}

impl<E, T> Timer<E, T> for Watchdog where E: Event + Send, T: Template<Event=E> {
    fn new(delta: Duration, correlator: Arc<Mutex<Correlator<E, T>>>) -> Self {
        let (tx, rx) = channel();

        let join_handle = thread::spawn(move || {
            loop {
                thread::sleep(delta);

                match rx.try_recv() {
                    Ok(StopEvent) | Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => (),
                }

                match correlator.lock() {
                    Ok(mut guard) => guard.elapse_time(delta),
                    Err(_) => break
                }
            }
        });

        Watchdog {
            sender: tx,
            _join_handle: join_handle
        }
    }
}

impl Drop for Watchdog {
    fn drop(&mut self) {
        let _ = self.sender.send(StopEvent);
    }
}
