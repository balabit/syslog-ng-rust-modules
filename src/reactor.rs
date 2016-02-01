use dispatcher::response::ResponseSender;
use context::ContextMap;

pub struct SharedData<'a> {
    pub responder: &'a mut ResponseSender,
    pub map: &'a mut ContextMap,
}

impl<'a> SharedData<'a> {
    pub fn new(map: &'a mut ContextMap, responder: &'a mut ResponseSender) -> SharedData<'a> {
        SharedData { map: map, responder: responder }
    }
}

pub trait EventHandler<T: Event, D> {
    fn handle_event(&mut self, event: T, shared_data: &mut D);
    fn handle(&self) -> T::Handle;
}

pub trait EventDemultiplexer {
    type Event: Event;
    fn select(&mut self) -> Option<Self::Event>;
}

pub trait Reactor {
    type Event: Event;
    fn handle_events(&mut self);
    fn register_handler(&mut self, handler: Box<for<'a> EventHandler<Self::Event, SharedData<'a>>>);
    fn remove_handler_by_handle(&mut self,
                                 handler: &<<Self as Reactor>::Event as Event>::Handle);
}

pub trait Event {
    type Handle;
    fn handle(&self) -> Self::Handle;
}
