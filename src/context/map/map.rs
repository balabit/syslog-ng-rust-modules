use std::collections::BTreeMap;
use std::fmt::Write;
use std::rc::Rc;

use action::ExecResult;
use Conditions;
use context::Event;
use Message;
use state::State;
use TimerEvent;
use context::base::BaseContext;
use context::EventHandler;

#[derive(Debug)]
pub struct MapContext {
    base: BaseContext,
    map: BTreeMap<String, State>,
    format_buffer: String
}

impl MapContext {
    pub fn new(conditions: Conditions) -> MapContext {
        MapContext {
            base: BaseContext::new(conditions),
            map: BTreeMap::new(),
            format_buffer: String::new()
        }
    }

    pub fn on_event(&mut self, event: Event) -> Option<Vec<ExecResult>> {
        match event {
            Event::Message(event) => self.on_message(event),
            Event::Timer(event) => self.on_timer(&event),
        }
    }

    pub fn on_timer(&mut self, event: &TimerEvent) -> Option<Vec<ExecResult>> {
        let mut result: Vec<ExecResult> = Vec::new();

        for (_, mut state) in self.map.iter_mut() {
            if let Some(commands) = self.base.on_timer(event, &mut state) {
                for i in commands.into_iter() {
                    result.push(i);
                }
            }
        }

        self.remove_closed_states();
        Some(result)
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

    pub fn on_message(&mut self, event: Rc<Message>) -> Option<Vec<ExecResult>> {
        self.format_context_id(&event);
        let result = self.update_state(event);
        self.format_buffer.clear();
        self.remove_closed_states();
        result
    }

    fn update_state(&mut self, event: Rc<Message>) -> Option<Vec<ExecResult>> {
        let id = self.format_buffer.clone();
        let state = self.map.entry(id).or_insert(State::new());
        self.base.on_message(event, state)
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

impl EventHandler<Event> for MapContext {
    fn handlers(&self) -> &[String] {
        self.patterns()
    }
    fn handle_event(&mut self, event: Event) -> Option<Vec<ExecResult>> {
        self.on_event(event)
    }
}

impl From<MapContext> for Box<EventHandler<Event>> {
    fn from(context: MapContext) -> Box<EventHandler<Event>> {
        Box::new(context)
    }
}
