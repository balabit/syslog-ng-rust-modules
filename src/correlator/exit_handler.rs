use std::sync::mpsc;

use Response;
use condition::Condition;
use dispatcher::request::Request;
use dispatcher::ResponseHandle;
use reactor::EventHandler;

pub struct ExitHandler {
    exits_received: u32,
    condition: Condition,
}

impl ExitHandler {
    pub fn new(condition: Condition) -> ExitHandler {
        ExitHandler {
            exits_received: 0,
            condition: condition,
        }
    }
}

impl EventHandler<Response, mpsc::Sender<Request>> for ExitHandler {
    fn handle_event(&mut self, event: Response, channel: &mut mpsc::Sender<Request>) {
        if let Response::Exit = event {
            self.exits_received += 1;
            let _ = channel.send(Request::Exit);

            if self.exits_received >= 1 {
                self.condition.activate()
            }
        }
    }
    fn handle(&self) -> ResponseHandle {
        ResponseHandle::Exit
    }
}
