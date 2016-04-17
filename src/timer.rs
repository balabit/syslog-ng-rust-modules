use std::sync::mpsc::{Sender, channel, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use Timer;
use correlation::correlator::Correlator;
use correlation::{Event, Template};
use std::thread;

enum WatchdogEvent {
    Cloned,
    Dropped
}

pub struct Watchdog {
    sender: Sender<WatchdogEvent>,
}

impl Clone for Watchdog {
    fn clone(&self) -> Watchdog {
        let _ = self.sender.send(WatchdogEvent::Cloned);
        Watchdog {
            sender: self.sender.clone(),
        }
    }
}

impl<E, T> Timer<E, T> for Watchdog where E: Event + Send, T: Template<Event=E> {
    fn new(delta: Duration, correlator: Arc<Mutex<Correlator<E, T>>>) -> Self {
        let correlator_for_timer = correlator.clone();
        let (tx, rx) = channel();

        thread::spawn(move || {
            let mut usage_count = 1;
            loop {
                thread::sleep(delta);

                match rx.try_recv() {
                    Ok(WatchdogEvent::Cloned) => usage_count += 1,
                    Ok(WatchdogEvent::Dropped) => usage_count -= 1,
                    Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => (),
                }

                if usage_count == 0 {
                    break;
                }

                match correlator_for_timer.lock() {
                    Ok(mut guard) => guard.elapse_time(delta),
                    Err(_) => break
                }
            }
        });

        Watchdog {
            sender: tx,
        }
    }
}

impl Drop for Watchdog {
    fn drop(&mut self) {
        let _ = self.sender.send(WatchdogEvent::Dropped);
    }
}
