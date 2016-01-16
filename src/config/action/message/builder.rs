use handlebars::Template;
use std::collections::BTreeMap;
use super::{MessageAction, ON_CLOSED_DEFAULT};

pub struct MessageActionBuilder {
    uuid: String,
    name: Option<String>,
    message: Template,
    values: BTreeMap<String, Template>,
    on_opened: Option<bool>,
    on_closed: Option<bool>
}

impl MessageActionBuilder {
    pub fn new<S: Into<String>>(uuid: S, message: Template) -> MessageActionBuilder {
        MessageActionBuilder {
            uuid: uuid.into(),
            name: None,
            message: message,
            values: BTreeMap::new(),
            on_opened: None,
            on_closed: ON_CLOSED_DEFAULT
        }
    }

    pub fn name<S: Into<String>>(&mut self, name: Option<S>) -> &mut MessageActionBuilder {
        self.name = name.map(|name| name.into());
        self
    }

    pub fn on_opened(&mut self, on_opened: Option<bool>) -> &mut MessageActionBuilder {
        self.on_opened = on_opened;
        self
    }

    pub fn on_closed(&mut self, on_closed: Option<bool>) -> &mut MessageActionBuilder {
        self.on_closed = on_closed;
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
            on_opened: self.on_opened,
            on_closed: self.on_closed,
        }
    }
}
