use std::sync::mpsc;
use std::thread;
use std::result::Result;

use action::ExecResult;
use action::ActionHandlers;
use super::{config, Command, Context, Dispatcher, Event, Message, Timer};

const TIMER_STEP: u32 = 100;

pub struct Correlator {
    action_handlers: ActionHandlers,
    dispatcher_input_channel: mpsc::Sender<Command>,
    dispatcher_output_channel: mpsc::Receiver<ExecResult>,
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
        Correlator::consume_results(&mut self.dispatcher_output_channel, &mut self.action_handlers);
        self.dispatcher_input_channel.send(Command::Dispatch(Event::Message(message)))
    }

    fn consume_results(channel: &mut mpsc::Receiver<ExecResult>, handlers: &mut ActionHandlers) {
        for i in channel.try_recv() {
            handlers.handle(i);
        }
    }

    fn consume_all_remaining_results(channel: &mut mpsc::Receiver<ExecResult>, handlers: &mut ActionHandlers) {
        for i in channel.recv() {
            handlers.handle(i);
        }
    }

    pub fn stop(self) -> thread::Result<()> {
        let Correlator {
            action_handlers: mut handlers,
            dispatcher_input_channel: input,
            dispatcher_output_channel: mut output,
            dispatcher_thread_handle: thread_handle } = self;

        Correlator::consume_results(&mut output, &mut handlers);
        let _ = input.send(Command::Exit);
        Correlator::consume_results(&mut output, &mut handlers);
        let join_result = thread_handle.join();
        Correlator::consume_all_remaining_results(&mut output, &mut handlers);
        join_result
    }
}
