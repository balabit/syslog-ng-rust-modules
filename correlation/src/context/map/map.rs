// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::time::Duration;

use state::State;
use context::base::BaseContext;
use Event;
use Template;
use Alert;

pub type ContextKey = Vec<(String, String)>;

pub struct MapContext<E, T> where E: Event, T: Template<Event=E> {
    base: BaseContext<E, T>,
    map: BTreeMap<ContextKey, State<E>>,
    context_id: Vec<String>,
}

impl<E, T> MapContext<E, T> where E: Event, T: Template<Event=E> {
    pub fn new(base: BaseContext<E, T>, context_id: Vec<String>) -> MapContext<E, T> {
        MapContext {
            base: base,
            map: BTreeMap::new(),
            context_id: context_id,
        }
    }

    #[allow(for_kv_map)]
    pub fn on_timer(&mut self, event: &Duration, responder: &mut VecDeque<Alert<E>>) {
        for (_, mut state) in &mut self.map {
            self.base.on_timer(&event, &mut state, responder);
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

    pub fn on_message(&mut self, event: E, responder: &mut VecDeque<Alert<E>>) {
        self.update_state(event, responder);
        self.remove_closed_states();
    }

    fn update_state(&mut self, event: E, responder: &mut VecDeque<Alert<E>>) {
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
