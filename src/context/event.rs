use std::rc::Rc;

use action::ExecResult;
use event;
use message::Message;
use timer::TimerEvent;

pub enum Event {
    Timer(TimerEvent),
    Message(Rc<Message>)
}

pub trait EventHandler<T> {
    fn handle_event(&mut self, T) -> Option<Vec<ExecResult>>;
    fn handlers(&self) -> &[String];
}

impl From<event::Event> for Event {
    fn from(event: event::Event) -> Event {
        match event {
            event::Event::Timer(event) => Event::Timer(event),
            event::Event::Message(event) => Event::Message(Rc::new(event))
        }
    }
}
