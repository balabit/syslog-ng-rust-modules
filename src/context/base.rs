use uuid::Uuid;
use std::rc::Rc;

use action::{Action, ExecResult};
use conditions::Conditions;
use config;
use message::Message;
use state::State;
use timer::TimerEvent;

#[derive(Debug)]
pub struct BaseContext {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<Action>
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

    pub fn on_timer(&self, event: &TimerEvent, state: &mut State) -> Option<Vec<ExecResult>> {
        if state.is_open() {
            state.update_timers(event);
            if self.conditions.is_closing(state) {
                return self.close_state(state);
            }
        }
        None
    }

    pub fn on_message(&self, event: Rc<Message>, state: &mut State) -> Option<Vec<ExecResult>> {
        if self.conditions.ignore_message(&event) {
            None
        } else {
            self.on_relevant_message(event, state)
        }
    }

    fn on_relevant_message(&self, event: Rc<Message>, state: &mut State) -> Option<Vec<ExecResult>> {
        if state.is_open() {
            state.add_message(event);
            if self.conditions.is_closing(state) {
                return self.close_state(state);
            }
        } else if self.conditions.is_opening(&event) {
            state.add_message(event);
            state.open();
        }
        None
    }

    fn close_state(&self, state: &mut State) -> Option<Vec<ExecResult>> {
        if self.actions.is_empty() {
            state.close();
            None
        } else {
            let commands = self.actions.iter().map(|action| action.execute(state, self)).collect();
            state.close();
            Some(commands)
        }
    }
}

impl From<config::Context> for BaseContext {
    fn from(config: config::Context) -> BaseContext {
        let config::Context{name, uuid, conditions, actions} = config;
        BaseContext {
            name: name,
            uuid: uuid,
            conditions: conditions,
            actions: actions
        }
    }
}
