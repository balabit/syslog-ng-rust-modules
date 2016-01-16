use handlebars::Template;
use std::collections::BTreeMap;
use super::MessageAction;
use config::action::ExecCondition;

pub struct MessageActionBuilder {
    uuid: String,
    name: Option<String>,
    message: Template,
    values: BTreeMap<String, Template>,
    when: ExecCondition
}

impl MessageActionBuilder {
    pub fn new<S: Into<String>>(uuid: S, message: Template) -> MessageActionBuilder {
        MessageActionBuilder {
            uuid: uuid.into(),
            name: None,
            message: message,
            values: BTreeMap::new(),
            when: ExecCondition::new()
        }
    }

    pub fn name<S: Into<String>>(&mut self, name: Option<S>) -> &mut MessageActionBuilder {
        self.name = name.map(|name| name.into());
        self
    }

    pub fn when(&mut self, when: ExecCondition) -> &mut MessageActionBuilder {
        self.when = when;
        self
    }

    pub fn values(&mut self, values: BTreeMap<String, Template>) -> &mut MessageActionBuilder {
        self.values = values;
        self
    }

    pub fn pair<S: Into<String>>(&mut self, key: S, value: Template) -> &mut MessageActionBuilder {
        self.values.insert(key.into(), value);
        self
    }

    pub fn build(&self) -> MessageAction {
        MessageAction {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            message: self.message.clone(),
            values: self.values.clone(),
            when: self.when.clone()
        }
    }
}
