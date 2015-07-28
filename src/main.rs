extern crate wheel_timer;

use wheel_timer::WheelTimer;

use std::collections::BTreeMap;
mod timer;

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
    wheel_of_times: WheelTimer<Box<Context>>
}

impl Correlator {
    pub fn new() -> Correlator {
        Correlator {
            contexts: BTreeMap::new(),
            wheel_of_times: WheelTimer::new(20)
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
                let expired_timers = self.wheel_of_times.tick();
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
    let mut timer = WheelTimer::new(10);
    timer.schedule(2, C(2, "2"));
    timer.schedule(4, C(4, "4"));
    timer.schedule(5, C(5,"5"));

    for i in 0..10 {
        for c in timer.tick() {
            timer.schedule(i + c.0, c);
            println!("{:?}", c);
        }
    }
}
