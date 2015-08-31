use uuid::Uuid;

use super::ActionType;
use super::Conditions;

mod deser;

#[derive(Debug, PartialEq, Eq)]
pub struct Context {
    pub name: Option<String>,
    pub uuid: Uuid,
    pub conditions: Conditions,
    pub actions: Vec<ActionType>
}

pub struct ContextBuilder {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<ActionType>
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

    pub fn actions(&mut self, actions: Vec<ActionType>) -> &mut ContextBuilder {
        self.actions = actions;
        self
    }

    pub fn name(&mut self, name: String) -> &mut ContextBuilder {
        self.name = Some(name);
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
