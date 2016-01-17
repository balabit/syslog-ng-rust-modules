use std::sync::mpsc;

use Response;
use message::Message;
use condition::Condition;
use dispatcher::request::Request;
use dispatcher::ResponseHandler;
use reactor::EventHandler;

pub struct ExitHandler {
    channel: mpsc::Sender<Request<Message>>,
    exits_received: u32,
    condition: Condition,
}

impl ExitHandler {
    pub fn new(condition: Condition, channel: mpsc::Sender<Request<Message>>) -> ExitHandler {
        ExitHandler {
            channel: channel,
            exits_received: 0,
            condition: condition,
        }
    }
}

impl EventHandler<Response, ()> for ExitHandler {
    fn handle_event(&mut self, event: Response, _: &mut ()) {
        if let Response::Exit = event {
            self.exits_received += 1;
            let _ = self.channel.send(Request::Exit);

            if self.exits_received >= 1 {
                self.condition.activate()
            }
        }
    }
    fn handler(&self) -> ResponseHandler {
        ResponseHandler::Exit
    }
}
