use handlebars::{Handlebars, Template};
use std::collections::BTreeMap;
use std::sync::Arc;

use message::Message;
use state::State;
use timer::TimerEvent;
use context::base::BaseContext;
use dispatcher::request::Request;
use dispatcher::response::ResponseSender;

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
            context_id: handlebars,
        }
    }

    pub fn on_event(&mut self, event: Request, responder: &mut ResponseSender) {
        trace!("MapContext: received event");
        match event {
            Request::Timer(event) => self.on_timer(&event, responder),
            Request::Message(message) => self.on_message(message, responder),
            _ => {}
        }
    }

    pub fn on_timer(&mut self, event: &TimerEvent, responder: &mut ResponseSender) {
        for (_, mut state) in &mut self.map {
            state.on_timer(event, &self.base, responder);
        }
        self.remove_closed_states();
    }

    fn get_closed_state_ids(&self) -> Vec<String> {
        self.map
            .iter()
            .filter_map(|(id, state)| {
                if !state.is_open() {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
    }

    fn remove_closed_states(&mut self) {
        for id in self.get_closed_state_ids() {
            let _ = self.map.remove(&id);
        }
    }

    pub fn on_message(&mut self, event: Arc<Message>, responder: &mut ResponseSender) {
        self.update_state(event, responder);
        self.remove_closed_states();
    }

    fn update_state(&mut self, event: Arc<Message>, responder: &mut ResponseSender) {
        if let Ok(id) = self.context_id.render(CONTEXT_ID, event.values()) {
            let state = self.map.entry(id).or_insert_with(State::new);
            state.on_message(event, &self.base, responder);
        } else {
            error!("Failed to render the context-id: {:?}",
                   self.context_id.get_template(&CONTEXT_ID.to_owned()));
        }
    }

    #[allow(dead_code)]
    pub fn is_open(&self) -> bool {
        !self.map.is_empty()
    }

    pub fn patterns(&self) -> &[String] {
        &self.base.conditions().patterns
    }
}
