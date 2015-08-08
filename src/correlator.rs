use std::sync::mpsc;
use std::thread;
use std::result::Result;

use action::ActionHandlers;
use super::{config, Command, CommandResult, Context, Dispatcher, Event, Message, Timer};

const TIMER_STEP: u32 = 100;


pub struct Correlator {
    action_handlers: ActionHandlers,
    dispatcher_input_channel: mpsc::Sender<Command>,
    dispatcher_output_channel: mpsc::Receiver<CommandResult>,
    dispatcher_thread_handle: thread::JoinHandle<()>
}

impl Correlator {
    pub fn new(contexts: Vec<config::Context>, action_handlers: ActionHandlers) -> Correlator {
        let (dispatcher_input_channel, rx) = mpsc::channel();
        let (dispatcher_output_channel_tx, dispatcher_output_channel_rx) = mpsc::channel();
        let _ = Timer::from_chan(TIMER_STEP, dispatcher_input_channel.clone());

        let handle = thread::spawn(move || {
            let mut dispatcher = Dispatcher::new(contexts, dispatcher_output_channel_tx);
            dispatcher.start_loop(rx);
        });

        Correlator {
            action_handlers: action_handlers,
            dispatcher_input_channel: dispatcher_input_channel,
            dispatcher_output_channel: dispatcher_output_channel_rx,
            dispatcher_thread_handle: handle
        }
    }

    pub fn push_message(&mut self, message: Message) -> Result<(), mpsc::SendError<Command>> {
        self.consume_results();
        self.dispatcher_input_channel.send(Command::Dispatch(Event::Message(message)))
    }

    fn consume_results(&mut self) {
        for i in self.dispatcher_output_channel.try_recv() {
            if let CommandResult::Dispatch(result) = i {
                self.action_handlers.handle(result);
            }
        }
    }

    fn stop_dispatcher(&mut self) {
        let _ = self.dispatcher_input_channel.send(Command::Exit);
        let mut exit_num = 0;
        loop {
            let value = self.dispatcher_output_channel.recv();
            match value {
                Ok(value) => {
                    match value {
                        CommandResult::Dispatch(result) => self.action_handlers.handle(result),
                        CommandResult::Exit => {
                            let _ = self.dispatcher_input_channel.send(Command::Exit);
                            exit_num += 1;
                            if exit_num >= 1 {
                                break;
                            }
                        }
                    }
                },
                Err(_) => {}
            }
        }
    }

    pub fn stop(mut self) -> thread::Result<()> {
        self.consume_results();
        self.stop_dispatcher();
        self.dispatcher_thread_handle.join()
    }
}
