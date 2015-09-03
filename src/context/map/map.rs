use uuid::Uuid;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::rc::Rc;

use Conditions;
use message::{Message};
use state::State;
use TimerEvent;
use context::base::BaseContext;
use context::event::EventHandler;
use dispatcher::request::{Request, InternalRequest};

pub struct MapContext {
    base: BaseContext,
    map: BTreeMap<String, State>,
    format_buffer: String
}

impl MapContext {
    pub fn new(uuid: Uuid, conditions: Conditions) -> MapContext {
        MapContext {
            base: BaseContext::new(uuid, conditions),
            map: BTreeMap::new(),
            format_buffer: String::new()
        }
    }

    pub fn on_event(&mut self, event: InternalRequest) {
        if let Request::Message(event) = event {
            self.on_message(event);
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
        self.format_context_id(&event);
        self.update_state(event);
        self.format_buffer.clear();
        self.remove_closed_states();
    }

    fn update_state(&mut self, event: Rc<Message>) {
        let id = self.format_buffer.clone();
        let state = self.map.entry(id).or_insert(State::new());
        self.base.on_message(event, state);
    }

    pub fn is_open(&mut self) -> bool {
        !self.map.is_empty()
    }

    fn format_context_id(&mut self, message: &Message) {
        let empty = String::new();
        let _ = self.format_buffer.write_str(message.get("HOST").unwrap_or(&empty));
        let _ = self.format_buffer.write_str(":");
        let _ = self.format_buffer.write_str(message.get("PROGRAM").unwrap_or(&empty));
        let _ = self.format_buffer.write_str(":");
        let _ = self.format_buffer.write_str(message.get("PID").unwrap_or(&empty));
    }

    pub fn patterns(&self) -> &[String] {
        &self.base.conditions().patterns
    }
}

impl From<BaseContext> for MapContext {
    fn from(context: BaseContext) -> MapContext {
        MapContext {
            base: context,
            map: BTreeMap::new(),
            format_buffer: String::new()
        }
    }
}

impl EventHandler<InternalRequest> for MapContext {
    fn handlers(&self) -> &[String] {
        self.patterns()
    }
    fn handle_event(&mut self, event: InternalRequest) {
        self.on_event(event);
    }
}

impl From<MapContext> for Box<EventHandler<InternalRequest>> {
    fn from(context: MapContext) -> Box<EventHandler<InternalRequest>> {
        Box::new(context)
    }
}
