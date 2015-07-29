#[macro_use]
extern crate maplit;
use std::collections::BTreeMap;

pub type Message = BTreeMap<String, String>;

#[test]
fn it_works() {
}

pub enum EventType {
    Timer(TimerEvent),
    Message(Message)
}

pub struct TimerEvent;

pub trait Context {
    fn on_event(&mut self, event: &EventType) {
        match *event {
            EventType::Timer(ref event) => self.on_timer(event),
            EventType::Message(ref event) => self.on_message(event),
        }
    }
    fn on_timer(&mut self, event: &TimerEvent) {}
    fn on_message(&mut self, event: &Message) {}
}

pub struct Correlator {
    contexts: BTreeMap<String, Box<Context>>,
}

impl Correlator {
    pub fn new() -> Correlator {
        Correlator {
            contexts: BTreeMap::new(),
        }
    }

    pub fn push_message(&mut self, message: Message) {
        self.push(EventType::Message(message))
    }

    pub fn push_timer(&mut self) {
        self.push(EventType::Timer(TimerEvent))
    }

    fn push(&mut self, event: EventType) {
        match event {
            EventType::Message(ref event) => {
                let uuid = event.get("uuid").unwrap();
                let contexts = self.contexts.get(uuid);
            },
            EventType::Timer(ref event) => {
            }
        }
    }
}

pub struct CorrelationContext {
    messages: Vec<Message>
}

impl Context for CorrelationContext {
    fn on_timer(&mut self, event: &TimerEvent) {
        println!("TimerEvent");
    }
    fn on_message(&mut self, event: &Message) {
        println!("MessageEvent");
    }
}

#[derive(Debug, Copy, Clone)]
struct C<'a>(
    usize,
    &'a str
);

fn main() {
}
