use uuid::Uuid;
use std::rc::Rc;

use action::Action;
use conditions::Conditions;
use message::Message;
use state::State;
use timer::TimerEvent;

pub struct BaseContext {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<Box<Action>>
}

impl BaseContext {
    pub fn new(uuid: Uuid, conditions: Conditions) -> BaseContext {
        BaseContext {
            name: None,
            uuid: uuid,
            conditions: conditions,
            actions: Vec::new()
        }
    }

    pub fn conditions(&self) -> &Conditions {
        &self.conditions
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn on_timer(&self, event: &TimerEvent, state: &mut State) {
        if state.is_open() {
            state.update_timers(event);
            if self.conditions.is_closing(state) {
                self.close_state(state);
            }
        }
    }

    pub fn on_message(&self, event: Rc<Message>, state: &mut State) {
        if state.is_open() {
            state.add_message(event);
            if self.conditions.is_closing(state) {
                self.close_state(state);
            }
        } else if self.conditions.is_opening(&event) {
            state.add_message(event);
            state.open();
        }
    }

    fn close_state(&self, state: &mut State) {
        for i in &self.actions {
            i.execute(state, self);
        }
        state.close();
    }
}

pub struct Builder {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<Box<Action>>
}

impl Builder {
    pub fn new(uuid: Uuid, conditions: Conditions) -> Builder {
        Builder {
            name: None,
            uuid: uuid,
            conditions: conditions,
            actions: Vec::new()
        }
    }

    pub fn name(mut self, name: Option<String>) -> Builder {
        self.name = name;
        self
    }

    pub fn actions(mut self, actions: Vec<Box<Action>>) -> Builder {
        self.actions = actions;
        self
    }

    pub fn build(self) -> BaseContext {
        let Builder {name, uuid, conditions, actions} = self;
        BaseContext {
            name: name,
            uuid: uuid,
            conditions: conditions,
            actions: actions
        }
    }
}
