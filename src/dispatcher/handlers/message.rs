use dispatcher::request::{Request, RequestHandle};
use context::context_map::StreamingIterator;
use reactor::{EventHandler, SharedData};

pub struct MessageEventHandler;

impl MessageEventHandler {
    pub fn new() -> MessageEventHandler {
        MessageEventHandler
    }
}

impl<'a> EventHandler<Request, SharedData<'a>> for MessageEventHandler {
    fn handle_event(&mut self, event: Request, data: &mut SharedData) {
        trace!("MessageEventHandler: handle_event()");
        if let Request::Message(event) = event {
            for i in event.ids() {
                let mut iter = data.map.contexts_iter_mut(i);
                while let Some(context) = iter.next() {
                    context.on_event(Request::Message(event.clone()), data.responder);
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
