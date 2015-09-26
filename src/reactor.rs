pub trait EventHandler<T: Event, D> {
    fn handle_event(&mut self, event: T, shared_data: &mut D);
    fn handler(&self) -> T::Handler;
}

pub trait EventDemultiplexer {
    type Event: Event;
    fn select(&mut self) -> Option<Self::Event>;
}

pub trait Reactor<D> {
    type Event: Event;
    fn handle_events(&mut self);
    fn register_handler(&mut self, handler: Box<EventHandler<Self::Event, D>>);
    fn remove_handler_by_handler(&mut self, handler: &<<Self as Reactor<D>>::Event as Event>::Handler);
}

pub trait Event {
    type Handler;
    fn handler(&self) -> Self::Handler;
}
