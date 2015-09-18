use std::collections::BTreeMap;

pub use self::builder::MessageBuilder;

mod builder;
#[cfg(test)]
mod test;
mod to_json;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    uuid: String,
    name: Option<String>,
    message: String,
    values: BTreeMap<String, String>
}

impl Message {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    pub fn ids(&self) -> IdIterator {
        IdIterator {
            message: self,
            state: 0
        }
    }
}

pub struct IdIterator<'a> {
    message: &'a Message,
    state: u8
}

impl<'a> Iterator for IdIterator<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                self.state += 1;
                Some(self.message.uuid())
            },
            1 => {
                self.state += 1;
                self.message.name()
            },
            _ => None
        }
    }
}
