use uuid::Uuid;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Message {
    uuid: PatternId,
    name: Option<PatternId>,
    data: BTreeMap<String, String>
}

impl Message {
    pub fn uuid(&self) -> &PatternId {
        &self.uuid
    }

    pub fn name(&self) -> Option<&PatternId> {
        self.name.as_ref()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

pub struct Builder {
    uuid: PatternId,
    name: Option<PatternId>,
    data: BTreeMap<String, String>
}

impl Builder {
    pub fn new(uuid: &str) -> Builder {
        Builder {
            uuid: PatternId::Uuid(Uuid::parse_str(uuid).unwrap()),
            name: None,
            data: BTreeMap::new()
        }
    }

    pub fn name(&mut self, name: String) -> &mut Builder {
        self.name = Some(PatternId::Name(name));
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PatternId {
    Uuid(Uuid),
    Name(String)
}
