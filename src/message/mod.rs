use std::collections::BTreeMap;

pub use self::builder::MessageBuilder;

mod builder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    uuid: String,
    name: Option<String>,
    values: BTreeMap<String, String>
}

impl Message {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }
}
