use std::rc::Rc;

use message::Message;
use reactor;
use timer::TimerEvent;

#[derive(Clone, Debug)]
pub enum Request<M> {
    Message(M),
    Timer(TimerEvent),
    Exit,
}

pub type InternalRequest = Request<Rc<Message>>;
pub type ExternalRequest = Request<Message>;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RequestHandle {
    Message,
    Timer,
    Exit,
}

impl reactor::Event for Request<Rc<Message>> {
    type Handle = RequestHandle;
    fn handle(&self) -> Self::Handle {
        match *self {
            Request::Message(_) => RequestHandle::Message,
            Request::Timer(_) => RequestHandle::Timer,
            Request::Exit => RequestHandle::Exit,
        }
    }
}

impl From<Request<Message>> for Request<Rc<Message>> {
    fn from(request: Request<Message>) -> Request<Rc<Message>> {
        match request {
            Request::Message(message) => Request::Message(Rc::new(message)),
            Request::Timer(event) => Request::Timer(event),
            Request::Exit => Request::Exit,
        }
    }
}
