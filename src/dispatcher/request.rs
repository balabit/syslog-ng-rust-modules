use std::sync::Arc;

use message::Message;
use reactor;
use timer::TimerEvent;

#[derive(Clone, Debug)]
pub enum Request {
    Message(Arc<Message>),
    Timer(TimerEvent),
    Exit,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RequestHandle {
    Message,
    Timer,
    Exit,
}

impl reactor::Event for Request {
    type Handle = RequestHandle;
    fn handle(&self) -> Self::Handle {
        match *self {
            Request::Message(_) => RequestHandle::Message,
            Request::Timer(_) => RequestHandle::Timer,
            Request::Exit => RequestHandle::Exit,
        }
    }
}
