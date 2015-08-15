use std::sync::mpsc::Receiver;

use dispatcher::request::Request;
use reactor::EventDemultiplexer;

pub struct Demultiplexer<T>(Receiver<T>);

impl<T> Demultiplexer<T> {
    pub fn new(receiver: Receiver<T>) -> Demultiplexer<T> {
        Demultiplexer(receiver)
    }
}

impl EventDemultiplexer for Demultiplexer<Request> {
    type Event = Request;
    fn select(&mut self) -> Option<Self::Event> {
        self.0.recv().ok()
    }
}
