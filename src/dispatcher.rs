use std::collections::BTreeMap;

use super::{Context, Event, Observer};

pub struct Dispatcher {
    contexts: BTreeMap<String, Vec<Context>>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        let contexts = btreemap!{
            "1".to_string() => vec!(Context::new()),
            "2".to_string() => vec!(Context::new()),
            "3".to_string() => vec!(Context::new()),
        };
        Dispatcher{
            contexts: contexts,
        }
    }

    pub fn dispatch(&mut self, event: Event) {
        match event {
            Event::Message(event) => {
                if let Some(uuid) = event.get("uuid") {
                    if let Some(mut contexts) = self.contexts.get_mut(uuid) {
                        for i in contexts.iter_mut() {
                            i.on_message(&event);
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
