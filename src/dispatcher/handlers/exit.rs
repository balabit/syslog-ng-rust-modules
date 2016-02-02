use dispatcher::response::ResponseSender;
use dispatcher::Response;
use dispatcher::request::{Request, RequestHandle};
use condition::Condition;
use reactor::{EventHandler, SharedData};

pub struct ExitEventHandler {
    condition: Condition,
    stops: u32,
}

impl ExitEventHandler {
    pub fn new(condition: Condition) -> ExitEventHandler {
        ExitEventHandler {
            condition: condition,
            stops: 0,
        }
    }
}

impl<'a> EventHandler<Request, SharedData<'a>> for ExitEventHandler {
    fn handle_event(&mut self, event: Request, data: &mut SharedData) {
        if let Request::Exit = event {
            self.stops += 1;
            data.responder.send_response(Response::Exit);

            if self.stops >= 2 {
                self.condition.activate();
            }
        } else {
            unreachable!("An ExitEventHandler should only receive Exit events");
        }
    }
    fn handle(&self) -> RequestHandle {
        RequestHandle::Exit
    }
}
