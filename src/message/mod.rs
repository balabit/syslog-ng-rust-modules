use std::collections::BTreeMap;

pub use self::builder::Builder;

mod builder;

#[derive(Clone, Debug)]
pub struct Message {
    uuid: String,
    name: Option<String>,
    data: BTreeMap<String, String>
}

impl Message {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}
