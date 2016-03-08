// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::BTreeMap;
use std::sync::Arc;

use message::Message;
use state::State;
use timer::TimerEvent;
use context::base::BaseContext;
use dispatcher::request::Request;
use dispatcher::response::ResponseSender;

pub type ContextKey = Vec<(String, String)>;

pub struct MapContext {
    base: BaseContext,
    map: BTreeMap<ContextKey, State>,
    context_id: Vec<String>,
}

impl MapContext {
    pub fn new(base: BaseContext, context_id: Vec<String>) -> MapContext {
        MapContext {
            base: base,
            map: BTreeMap::new(),
            context_id: context_id,
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

    #[allow(for_kv_map)]
    pub fn on_timer(&mut self, event: &TimerEvent, responder: &mut ResponseSender) {
        for (_, mut state) in &mut self.map {
            self.base.on_timer(event, &mut state, responder);
        }
        self.remove_closed_states();
    }

    fn get_closed_state_ids(&self) -> Vec<ContextKey> {
        self.map
            .iter()
            .filter_map(|(id, state)| {
                if state.is_open() {
                    None
                } else {
                    Some(id.clone())
                }
            })
            .collect::<Vec<ContextKey>>()
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
        let key: ContextKey = self.context_id.iter().map(|key| {
                (key.to_owned(), event.get(&key).map_or_else(|| "".to_owned(), |value| value.to_owned()))
            }).collect();
        let mut state = self.map.entry(key).or_insert_with(State::new);
        self.base.on_message(event, &mut state, responder);
    }

    #[allow(dead_code)]
    pub fn is_open(&self) -> bool {
        !self.map.is_empty()
    }

    pub fn patterns(&self) -> &[String] {
        &self.base.patterns
    }
}
