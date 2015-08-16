use std::rc::Rc;

use message::Message;
use reactor;
use timer::TimerEvent;

#[derive(Clone, Debug)]
pub enum Request<M> {
    Message(M),
    Timer(TimerEvent),
    Exit
}

pub type InternalRequest = Request<Rc<Message>>;
pub type ExternalRequest = Request<Message>;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RequestHandler {
    Message,
    Timer,
    Exit
}

impl reactor::Event for Request<Rc<Message>> {
    type Handler = RequestHandler;
    fn handler(&self) -> Self::Handler {
        match *self {
            Request::Message(_) => RequestHandler::Message,
            Request::Timer(_) => RequestHandler::Timer,
            Request::Exit => RequestHandler::Exit,
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
