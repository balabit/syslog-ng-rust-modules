use std::sync::mpsc::Receiver;

use dispatcher::request::{InternalRequest, ExternalRequest};
use reactor::EventDemultiplexer;
use condition::Condition;

pub struct Demultiplexer<T> {
    channel: Receiver<T>,
    condition: Condition
}

impl<T> Demultiplexer<T> {
    pub fn new(channel: Receiver<T>, condition: Condition) -> Demultiplexer<T> {
        Demultiplexer {channel: channel, condition: condition}
    }
}

impl EventDemultiplexer for Demultiplexer<ExternalRequest> {
    type Event = InternalRequest;
    fn select(&mut self) -> Option<Self::Event> {
        if !self.condition.is_active() {
            let data = self.channel.recv().ok();
            data.map(|request| request.into())
        } else {
            None
        }
    }
}
