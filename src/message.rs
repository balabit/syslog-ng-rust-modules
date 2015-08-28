use std::collections::BTreeMap;

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
}

pub struct Builder {
    uuid: String,
    name: Option<String>,
    data: BTreeMap<String, String>
}

impl Builder {
    pub fn new(uuid: &str) -> Builder {
        Builder {
            uuid: uuid.to_string(),
            name: None,
            data: BTreeMap::new()
        }
    }

    pub fn name(&mut self, name: String) -> &mut Builder {
        self.name = Some(name);
        self
    }

    pub fn pair(&mut self, key: String, value: String) -> &mut Builder {
        self.data.insert(key, value);
        self
    }

    pub fn build(&self) -> Message {
        Message {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            data: self.data.clone()
        }
    }
}
