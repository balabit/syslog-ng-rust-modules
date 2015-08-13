pub trait EventHandler<T> {
    fn handle_event(&mut self, event: T);
}

pub trait EventDemultiplexer {
    type Event;
    fn select(&mut self) -> Option<Self::Event>;
}

pub trait Reactor {
    type Event;
    fn handle_events(&mut self);
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event>>);
    fn remove_handler(&mut self, handler: &EventHandler<Self::Event>);
}
