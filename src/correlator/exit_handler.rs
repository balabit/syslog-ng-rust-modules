use std::sync::mpsc;

use Response;
use condition::Condition;
use dispatcher::request::Request;
use dispatcher::ResponseHandle;
use reactor::EventHandler;

pub struct ExitHandler {
    channel: mpsc::Sender<Request>,
    exits_received: u32,
    condition: Condition,
}

impl ExitHandler {
    pub fn new(condition: Condition, channel: mpsc::Sender<Request>) -> ExitHandler {
        ExitHandler {
            channel: channel,
            exits_received: 0,
            condition: condition,
        }
    }
}

impl EventHandler<Response, mpsc::Sender<Request>> for ExitHandler {
    fn handle_event(&mut self, event: Response, _: &mut mpsc::Sender<Request>) {
        if let Response::Exit = event {
            self.exits_received += 1;
            let _ = self.channel.send(Request::Exit);

            if self.exits_received >= 1 {
                self.condition.activate()
            }
        }
    }
    fn handle(&self) -> ResponseHandle {
        ResponseHandle::Exit
    }
}
