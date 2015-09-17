use handlebars::Template;
use std::collections::BTreeMap;
use super::MessageAction;

pub struct MessageActionBuilder {
    uuid: String,
    name: Option<String>,
    message: Template,
    values: BTreeMap<String, String>
}

impl MessageActionBuilder {
    pub fn new(uuid: &str, message: Template) -> MessageActionBuilder {
        MessageActionBuilder {
            uuid: uuid.to_string(),
            name: None,
            message: message,
            values: BTreeMap::new()
        }
    }

    pub fn name(&mut self, name: &str) -> &mut MessageActionBuilder {
        self.name = Some(name.to_string());
        self
    }

    pub fn pair(&mut self, key: &str, value: &str) -> &mut MessageActionBuilder {
        self.values.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(&self) -> MessageAction {
        MessageAction {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            message: self.message.clone(),
            values: self.values.clone()
        }
    }
}
