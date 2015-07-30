use std::sync::mpsc;
use std::thread;
use std::result::Result;

use super::{config, Command, Context, Dispatcher, Event, Message, Timer};

const TIMER_STEP: u32 = 100;

pub struct Correlator {
    tx: mpsc::Sender<Command>,
    dispatcher_thread_handle: thread::JoinHandle<()>
}

impl Correlator {
    pub fn new(contexts: Vec<config::Context>) -> Correlator {
        let (tx, rx) = mpsc::channel();
        let _ = Timer::from_chan(TIMER_STEP, tx.clone());

        let handle = thread::spawn(move || {
            let mut dispatcher = Dispatcher::new(contexts);

            for i in rx.iter() {
                match i {
                    Command::Dispatch(event) => dispatcher.dispatch(event),
                    Command::Exit => break
                }
            }
        });

        Correlator {
            tx: tx,
            dispatcher_thread_handle: handle
        }
    }

    pub fn push_message(&mut self, message: Message) -> Result<(), mpsc::SendError<Command>> {
        self.tx.send(Command::Dispatch(Event::Message(message)))
    }

    pub fn stop(self) -> thread::Result<()> {
        let _ = self.tx.send(Command::Exit);
        self.dispatcher_thread_handle.join()
    }
}
