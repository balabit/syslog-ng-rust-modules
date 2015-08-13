pub trait EventHandler<T> {
    type Handler;
    fn handle_event(&mut self, event: T);
    fn handler(&self) -> Self::Handler;
}

pub trait EventDemultiplexer {
    type Event;
    fn select(&mut self) -> Option<Self::Event>;
}

pub trait Reactor {
    type Event: Event;
    type Handler;
    fn handle_events(&mut self);
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event, Handler=Self::Handler>>);
    fn remove_handler(&mut self, handler: &EventHandler<Self::Event, Handler=Self::Handler>);
}

pub trait Event {
    type Handler;
    fn handler(&self) -> Self::Handler;
}
