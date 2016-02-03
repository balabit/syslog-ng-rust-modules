use std::sync::mpsc::Receiver;

use dispatcher::request::Request;
use reactor::EventDemultiplexer;

pub struct Demultiplexer<T> {
    channel: Receiver<T>,
    stops: u32
}

impl<T> Demultiplexer<T> {
    pub fn new(channel: Receiver<T>) -> Demultiplexer<T> {
        Demultiplexer {
            channel: channel,
            stops: 0
        }
    }
}

impl EventDemultiplexer for Demultiplexer<Request> {
    type Event = Request;
    fn select(&mut self) -> Option<Self::Event> {
        let data = self.channel.recv().ok();

        if let Some(Request::Exit) = data {
            if self.stops >= 1 {
                None
            } else {
                self.stops += 1;
                Some(Request::Exit)
            }
        } else {
            data
        }
    }
}
