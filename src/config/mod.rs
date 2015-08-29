use uuid::Uuid;

use super::Action;
use super::Conditions;

#[derive(Debug)]
pub struct Context {
    pub name: Option<String>,
    pub uuid: Uuid,
    pub conditions: Conditions,
    pub actions: Vec<Action>
}

pub struct ContextBuilder {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<Action>
}

impl ContextBuilder {
    pub fn new(uuid: Uuid, conditions: Conditions) -> ContextBuilder {
        ContextBuilder {
            name: None,
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
            name: self.name.clone(),
            uuid: self.uuid.clone(),
            conditions: self.conditions.clone(),
            actions: self.actions.clone()
        }
    }
}
