use super::Conditions;

pub struct Context {
    pub conditions: Conditions
}

impl Context {
    pub fn new(conditions: Conditions) -> Context {
        Context {
            conditions: conditions
        }
    }
}
