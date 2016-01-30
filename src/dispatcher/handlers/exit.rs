use context::ContextMap;
use dispatcher::response::ResponseSender;
use dispatcher::Response;
use dispatcher::request::{Request, RequestHandle};
use condition::Condition;
use reactor::EventHandler;

pub struct ExitEventHandler {
    condition: Condition,
    response_handler: Box<ResponseSender<Response>>,
    stops: u32,
}

impl ExitEventHandler {
    pub fn new(condition: Condition,
               response_handler: Box<ResponseSender<Response>>)
               -> ExitEventHandler {
        ExitEventHandler {
            condition: condition,
            response_handler: response_handler,
            stops: 0,
        }
    }
}

impl EventHandler<Request, ContextMap> for ExitEventHandler {
    fn handle_event(&mut self, event: Request, _: &mut ContextMap) {
        if let Request::Exit = event {
            self.stops += 1;
            self.response_handler.send_response(Response::Exit);

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
