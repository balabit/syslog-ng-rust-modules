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
