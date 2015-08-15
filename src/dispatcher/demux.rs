use std::sync::mpsc::Receiver;

use dispatcher::request::Request;
use reactor::EventDemultiplexer;

pub struct Demultiplexer<T>(Receiver<T>);

impl EventDemultiplexer for Demultiplexer<Request> {
    type Event = Request;
    fn select(&mut self) -> Option<Self::Event> {
        self.0.recv().ok()
    }
}
