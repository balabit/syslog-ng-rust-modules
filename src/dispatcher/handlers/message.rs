use dispatcher::request::{Request, RequestHandle};
use context::ContextMap;
use context::context_map::StreamingIterator;
use reactor;

pub struct MessageEventHandler;

impl MessageEventHandler {
    pub fn new() -> MessageEventHandler {
        MessageEventHandler
    }
}

impl reactor::EventHandler<Request, ContextMap> for MessageEventHandler {
    fn handle_event(&mut self, event: Request, data: &mut ContextMap) {
        trace!("MessageEventHandler: handle_event()");
        if let Request::Message(event) = event {
            for i in event.ids() {
                let mut iter = data.contexts_iter_mut(i);
                while let Some(context) = iter.next() {
                    context.on_event(Request::Message(event.clone()));
                }
            }
        } else {
            unreachable!("MessageEventHandler should only handle Message events");
        }
    }
    fn handle(&self) -> RequestHandle {
        RequestHandle::Message
    }
}
