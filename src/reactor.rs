pub trait EventHandler<T: Event> {
    fn handle_event(&mut self, event: T);
    fn handler(&self) -> T::Handler;
}

pub trait EventDemultiplexer {
    type Event: Event;
    fn select(&mut self) -> Option<Self::Event>;
}

pub trait Reactor {
    type Event: Event;
    type Handler;
    fn handle_events(&mut self);
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event>>);
    fn remove_handler_by_handler(&mut self, handler: &Self::Handler);
}

pub trait Event {
    type Handler;
    fn handler(&self) -> Self::Handler;
}
