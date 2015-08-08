use std::sync::mpsc;
use std::thread;
use std::result::Result;

use action::ActionCommand;
use action::ActionHandlers;
use super::{config, Command, Context, Dispatcher, Event, Message, Timer};

const TIMER_STEP: u32 = 100;

pub struct Correlator {
    action_handlers: ActionHandlers,
    dispatcher_input_channel: mpsc::Sender<Command>,
    dispatcher_output_channel: mpsc::Receiver<ActionCommand>,
    dispatcher_thread_handle: thread::JoinHandle<()>
}

impl Correlator {
    pub fn new(contexts: Vec<config::Context>, action_handlers: ActionHandlers) -> Correlator {
        let (dispatcher_input_channel, rx) = mpsc::channel();
        let (dispatcher_output_channel_tx, dispatcher_output_channel_rx) = mpsc::channel();
        let _ = Timer::from_chan(TIMER_STEP, dispatcher_input_channel.clone());

        let handle = thread::spawn(move || {
            let mut dispatcher = Dispatcher::new(contexts, dispatcher_output_channel_tx);

            for i in rx.iter() {
                match i {
                    Command::Dispatch(event) => dispatcher.dispatch(event),
                    Command::Exit => break
                }
            }
        });

        Correlator {
            action_handlers: action_handlers,
            dispatcher_input_channel: dispatcher_input_channel,
            dispatcher_output_channel: dispatcher_output_channel_rx,
            dispatcher_thread_handle: handle
        }
    }

    pub fn push_message(&mut self, message: Message) -> Result<(), mpsc::SendError<Command>> {
        for i in self.dispatcher_output_channel.try_recv() {
            self.action_handlers.handle(i);
        }

        self.dispatcher_input_channel.send(Command::Dispatch(Event::Message(message)))
    }

    pub fn stop(self) -> thread::Result<()> {
        let _ = self.dispatcher_input_channel.send(Command::Exit);
        self.dispatcher_thread_handle.join()
    }
}
