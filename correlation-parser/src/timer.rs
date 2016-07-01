use std::sync::mpsc::{Sender, channel, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use Timer;
use correlation::correlator::Correlator;
use correlation::{Event, Template};
use std::thread::{self, JoinHandle};

enum ControlEvent {
    Stop,
    Park,
    UnPark
}

pub struct Watchdog {
    sender: Sender<ControlEvent>,
    _join_handle: JoinHandle<()>,
}

impl Watchdog {
    pub fn schedule<F>(delta: Duration, mut user_callback: F) -> Self where F: 'static + FnMut() + Send {
        let (tx, rx) = channel();

        let join_handle = thread::spawn(move || {
            let mut is_parking = true;

            let mut dummy_callback = || ();

            loop {
                if is_parking {
                    // we may wake up spuriously from park()
                    ::std::thread::park();

                    match Watchdog::handle_control_event(rx.try_recv(), &mut dummy_callback) {
                        Ok(should_park) => is_parking = should_park,
                        Err(_) => break
                    }
                } else {
                    match Watchdog::handle_control_event(rx.try_recv(), &mut user_callback) {
                        Ok(should_park) => is_parking = should_park,
                        Err(_) => break
                    }
                    thread::sleep(delta);
                }
            }
        });

        Watchdog {
            sender: tx,
            _join_handle: join_handle,
        }
    }

    fn handle_control_event<F>(event: Result<ControlEvent, TryRecvError>, cb: &mut F) -> Result<bool, ()> where F: 'static + FnMut() + Send {
        match event {
            Ok(ControlEvent::Stop) | Err(TryRecvError::Disconnected) => Err(()),
            Ok(ControlEvent::Park) => Ok(true),
            Ok(ControlEvent::UnPark) => Ok(false),
            Err(TryRecvError::Empty) => {
                cb();
                Ok(false)
            }
        }
    }
}

impl<E, T> Timer<E, T> for Watchdog where E: Event + Send, T: Template<Event=E> {
    fn new(delta: Duration, correlator: Arc<Mutex<Correlator<E, T>>>) -> Self {
        Watchdog::schedule(delta, move || {
            let _ = match correlator.lock() {
                Ok(mut guard) => guard.elapse_time(delta),
                Err(_) => ()
            };
        })
    }

    fn start(&self) {
        let _ = self.sender.send(ControlEvent::UnPark);
        self._join_handle.thread().unpark();
    }

    fn stop(&self) {
        let _ = self.sender.send(ControlEvent::Park);
    }
}

impl Drop for Watchdog {
    fn drop(&mut self) {
        let _ = self.sender.send(ControlEvent::UnPark);
        self._join_handle.thread().unpark();
        let _ = self.sender.send(ControlEvent::Stop);
    }
}
