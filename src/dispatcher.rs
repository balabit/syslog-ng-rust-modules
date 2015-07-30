use std::collections::BTreeMap;
use std::rc::Rc;

use super::{context, Context, Event};

pub struct Dispatcher {
    contexts: BTreeMap<String, Vec<Context>>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        let contexts = btreemap!{
            "1".to_string() => vec!(context::Builder::new(100).build()),
            "2".to_string() => vec!(context::Builder::new(100).build()),
            "3".to_string() => vec!(context::Builder::new(100).build()),
        };
        Dispatcher {
            contexts: contexts,
        }
    }

    pub fn dispatch(&mut self, event: Event) {
        match event {
            Event::Message(event) => {
                let event = Rc::new(event);
                if let Some(uuid) = event.get("uuid") {
                    if let Some(mut contexts) = self.contexts.get_mut(uuid) {
                        for i in contexts.iter_mut() {
                            i.on_message(event.clone());
                        }
                    }
                }
            },
            Event::Timer(ref event) => {
                for (_, contexts) in self.contexts.iter_mut() {
                    for context in contexts.iter_mut() {
                        context.on_timer(event);
                    }
                }
            }
        }
    }
}
