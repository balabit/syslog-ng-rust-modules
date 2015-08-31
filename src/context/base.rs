use uuid::Uuid;
use std::rc::Rc;

use action::{Action, ActionType};
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
        state.close();
    }

    fn box_the_actions(actions: Vec<ActionType>) -> Vec<Box<Action>> {
        let mut boxed_actions = Vec::new();
        for i in actions.into_iter() {
            let action: Box<Action> = match i {
                ActionType::Message(action) => {
                    Box::new(action)
                }
            };

            boxed_actions.push(action);
        }
        boxed_actions
    }
}

impl From<config::Context> for BaseContext {
    fn from(config: config::Context) -> BaseContext {
        let config::Context{name, uuid, conditions, actions} = config;
        let boxed_actions = BaseContext::box_the_actions(actions);
        BaseContext {
            name: name,
            uuid: uuid,
            conditions: conditions,
            actions: boxed_actions
        }
    }
}
