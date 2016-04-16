use uuid::Uuid;

use Event;
use Template;
use Conditions;
use ActionType;
use context::BaseContext;

pub struct BaseContextBuilder<E, T> where E: Event, T: Template<Event=E> {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<ActionType<T>>,
    patterns: Vec<String>
}

impl<E, T> BaseContextBuilder<E, T> where E: Event, T: Template<Event=E> {
    pub fn new(uuid: Uuid, conditions: Conditions) -> BaseContextBuilder<E, T> {
        BaseContextBuilder {
            name: None,
            uuid: uuid,
            conditions: conditions,
            actions: Vec::new(),
            patterns: Vec::new()
        }
    }

    pub fn name(mut self, name: Option<String>) -> BaseContextBuilder<E, T> {
        self.name = name;
        self
    }

    pub fn actions(mut self, actions: Vec<ActionType<T>>) -> BaseContextBuilder<E, T> {
        self.actions = actions;
        self
    }

    pub fn patterns(mut self, patterns: Vec<String>) -> BaseContextBuilder<E, T> {
        self.patterns = patterns;
        self
    }
    pub fn build(self) -> BaseContext<E, T> {
        let BaseContextBuilder {name, uuid, conditions, actions, patterns} = self;
        BaseContext {
            name: name,
            uuid: uuid,
            conditions: conditions,
            actions: actions,
            patterns: patterns
        }
    }
}
