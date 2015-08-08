use super::Action;
use super::Conditions;

pub struct Context {
    pub conditions: Conditions,
    pub actions: Vec<Action>
}

impl Context {
    pub fn new(conditions: Conditions) -> Context {
        Context {
            conditions: conditions,
            actions: Vec::new()
        }
    }
}

pub struct ContextBuilder {
    conditions: Conditions,
    actions: Vec<Action>
}

impl ContextBuilder {
    pub fn new(conditions: Conditions) -> ContextBuilder {
        ContextBuilder {
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
            conditions: self.conditions.clone(),
            actions: self.actions.clone()
        }
    }
}
