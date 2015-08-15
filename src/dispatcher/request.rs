use ::Event;
use reactor;

#[derive(Debug)]
pub enum Request {
    Event(Event),
    Exit
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RequestHandler {
    Event,
    Exit
}

impl reactor::Event for Request {
    type Handler = RequestHandler;
    fn handler(&self) -> Self::Handler {
        match *self {
            Request::Event(_) => RequestHandler::Event,
            Request::Exit => RequestHandler::Exit,
        }
    }
}
