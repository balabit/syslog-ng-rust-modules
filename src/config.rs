use super::Conditions;

pub struct Context {
    pub conditions: Conditions,
    pub patterns: Vec<String>
}

impl Context {
    pub fn new(conditions: Conditions) -> Context {
        Context {
            conditions: conditions,
            patterns: Vec::new()
        }
    }
}
