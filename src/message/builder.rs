use std::collections::BTreeMap;
use super::Message;

pub struct Builder {
    uuid: String,
    name: Option<String>,
    values: BTreeMap<String, String>
}

impl Builder {
    pub fn new(uuid: &str) -> Builder {
        Builder {
            uuid: uuid.to_string(),
            name: None,
            values: BTreeMap::new()
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Builder {
        self.name = Some(name.to_string());
        self
    }

    pub fn pair(&mut self, key: &str, value: &str) -> &mut Builder {
        self.values.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(&self) -> Message {
        Message {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            values: self.values.clone()
        }
    }
}
