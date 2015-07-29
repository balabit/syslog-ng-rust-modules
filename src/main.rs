#[macro_use]
extern crate maplit;

use std::collections::BTreeMap;
use std::sync::mpsc;
use std::thread;

const TIMER_STEP: u32 = 100;

pub type Message = BTreeMap<String, String>;

#[derive(Debug)]
pub enum EventType {
    Timer(TimerEvent),
    Message(Message)
}

#[derive(Debug)]
pub struct TimerEvent;

pub struct Timer;

impl Timer {
    pub fn from_chan(ms: u32, tx: mpsc::Sender<EventType>) {
        thread::spawn(move || {
            loop {
                thread::sleep_ms(ms);
                if tx.send(EventType::Timer(TimerEvent)).is_err() {
                    break;
                }
            }
        });
    }
}

pub struct Dispatcher {
    contexts: BTreeMap<String, Box<Vec<Context>>>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        let contexts = btreemap!{
            "1".to_string() => Context::new(),
            "2".to_string() => Context::new(),
            "3".to_string() => Context::new(),
        };
        Dispatcher{
            contexts: BTreeMap::new(),
        }
    }

    fn dispatch(&mut self, event: EventType) {
        match event {
            EventType::Message(event) => {
                if let Some(uuid) = event.get("uuid") {
                    if let Some(mut contexts) = self.contexts.get_mut(uuid) {
                        for i in contexts.iter_mut() {
                            i.on_message(&event);
                        }
                    }
                }
            },
            EventType::Timer(ref event) => {
            }
        }
    }
}

pub struct Correlator {
    tx: mpsc::Sender<EventType>
}

impl Correlator {
    pub fn new() -> Correlator {
        let (tx, rx) = mpsc::channel();

        let timer = Timer::from_chan(TIMER_STEP, tx.clone());

        thread::spawn(move || {
            let mut dispatcher = Dispatcher::new();

            for i in rx.iter() {
                println!("{:?}", i);
                dispatcher.dispatch(i)
            }
        });

        Correlator{
            tx: tx
        }
    }

    pub fn push_message(&mut self, message: Message) {
        self.tx.send(EventType::Message(message));
    }

}

pub struct Context {
    messages: Vec<Message>
}

impl Context {
    fn new() -> Context {
        Context{
            messages: Vec::new()
        }
    }

    fn on_event(&mut self, event: &EventType) {
        match *event {
            EventType::Timer(ref event) => self.on_timer(event),
            EventType::Message(ref event) => self.on_message(event),
        }
    }
    fn on_timer(&mut self, event: &TimerEvent) {
        println!("timer event");
    }
    fn on_message(&mut self, event: &Message) {
        println!("message event");
    }
}

fn main() {
    let mut correlator = Correlator::new();
    let mut msg1 = btreemap!{
        "uuid".to_string() => "1".to_string(),
        "uuid".to_string() => "2".to_string(),
    };
    correlator.push_message(msg1);
}
