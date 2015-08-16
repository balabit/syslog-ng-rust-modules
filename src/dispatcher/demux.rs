use std::sync::mpsc::Receiver;

use dispatcher::request::{InternalRequest, ExternalRequest};
use reactor::EventDemultiplexer;

pub struct Demultiplexer<T>(Receiver<T>);

impl<T> Demultiplexer<T> {
    pub fn new(receiver: Receiver<T>) -> Demultiplexer<T> {
        Demultiplexer(receiver)
    }
}

impl EventDemultiplexer for Demultiplexer<ExternalRequest> {
    type Event = InternalRequest;
    fn select(&mut self) -> Option<Self::Event> {
        let data = self.0.recv().ok();
        data.map(|request| request.into())
    }
}
