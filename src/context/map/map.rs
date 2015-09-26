use handlebars::{
    Handlebars,
    Template,
};
use std::collections::BTreeMap;
use std::rc::Rc;

use message::{Message};
use state::State;
use timer::TimerEvent;
use context::base::BaseContext;
use dispatcher::request::{Request, InternalRequest};

const CONTEXT_ID: &'static str = ".context.id";

pub struct MapContext {
    base: BaseContext,
    map: BTreeMap<String, State>,
    context_id: Handlebars,
}

impl MapContext {
    pub fn new(base: BaseContext, context_id: Template) -> MapContext {
        let mut handlebars = Handlebars::new();
        handlebars.register_template(CONTEXT_ID, context_id);
        MapContext {
            base: base,
            map: BTreeMap::new(),
            context_id: handlebars
        }
    }

    pub fn on_event(&mut self, event: InternalRequest) {
        trace!("MapContext: received event");
        match event {
            Request::Timer(event) => {
                self.on_timer(&event)
            },
            Request::Message(message) => {
                self.on_message(message)
            },
            _ => {}
        }
    }

    pub fn on_timer(&mut self, event: &TimerEvent) {
        for (_, mut state) in self.map.iter_mut() {
            self.base.on_timer(event, &mut state);

        }
        self.remove_closed_states();
    }

    fn get_closed_state_ids(&self) -> Vec<String> {
        self.map.iter().filter_map(|(id, state)| {
            if !state.is_open() {
                Some(id.clone())
            } else {
                None
            }
        }).collect::<Vec<String>>()
    }

    fn remove_closed_states(&mut self) {
        for id in self.get_closed_state_ids() {
            let _ = self.map.remove(&id);
        }
    }

    pub fn on_message(&mut self, event: Rc<Message>) {
        self.update_state(event);
        self.remove_closed_states();
    }

    fn update_state(&mut self, event: Rc<Message>) {
        let id = self.context_id.render(CONTEXT_ID, event.values()).ok().expect("Failed to render the compiled Handlebars template");
        let state = self.map.entry(id).or_insert(State::new());
        self.base.on_message(event, state);
    }

    pub fn is_open(&mut self) -> bool {
        !self.map.is_empty()
    }

    pub fn patterns(&self) -> &[String] {
        &self.base.conditions().patterns
    }
}
