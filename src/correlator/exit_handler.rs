use std::sync::mpsc;

use Response;
use dispatcher::request::Request;
use dispatcher::ResponseHandle;
use reactor::EventHandler;

pub struct ExitHandler;

impl ExitHandler {
    pub fn new() -> ExitHandler {
        ExitHandler
    }
}

impl EventHandler<Response, mpsc::Sender<Request>> for ExitHandler {
    fn handle_event(&mut self, event: Response, channel: &mut mpsc::Sender<Request>) {
        if let Response::Exit = event {
            let _ = channel.send(Request::Exit);
        }
    }
    fn handle(&self) -> ResponseHandle {
        ResponseHandle::Exit
    }
}
