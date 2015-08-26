use uuid::Uuid;

use super::Action;
use super::Conditions;

pub struct Context {
    pub uuid: Uuid,
    pub conditions: Conditions,
    pub actions: Vec<Action>
}

impl Context {
    pub fn new(uuid: Uuid, conditions: Conditions) -> Context {
        Context {
            uuid: uuid,
            conditions: conditions,
            actions: Vec::new()
        }
    }
}

pub struct ContextBuilder {
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<Action>
}

impl ContextBuilder {
    pub fn new(uuid: Uuid, conditions: Conditions) -> ContextBuilder {
        ContextBuilder {
            uuid: uuid,
            conditions: conditions,
            actions: Vec::new()
        }
    }

    pub fn actions(&mut self, actions: Vec<Action>) -> &mut ContextBuilder {
        self.actions = actions;
        self
    }

    pub fn build(&self) -> Context {
        Context {
            uuid: self.uuid.clone(),
            conditions: self.conditions.clone(),
            actions: self.actions.clone()
        }
    }
}
