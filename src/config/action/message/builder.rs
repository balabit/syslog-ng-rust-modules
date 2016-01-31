use handlebars::Template;
use handlebars::Handlebars;
use super::MessageAction;
use super::InjectMode;
use config::action::ExecCondition;

pub struct MessageActionBuilder {
    uuid: String,
    name: Option<String>,
    message: Template,
    values: Handlebars,
    when: ExecCondition,
    inject_mode: InjectMode,
}

impl MessageActionBuilder {
    pub fn new<S: Into<String>>(uuid: S, message: Template) -> MessageActionBuilder {
        MessageActionBuilder {
            uuid: uuid.into(),
            name: None,
            message: message,
            values: Handlebars::new(),
            when: ExecCondition::new(),
            inject_mode: Default::default(),
        }
    }

    pub fn name<S: Into<String>>(mut self, name: Option<S>) -> MessageActionBuilder {
        self.name = name.map(|name| name.into());
        self
    }

    pub fn when(mut self, when: ExecCondition) -> MessageActionBuilder {
        self.when = when;
        self
    }

    pub fn values(mut self, values: Handlebars) -> MessageActionBuilder {
        self.values = values;
        self
    }

    pub fn pair<S: AsRef<str>>(mut self, key: S, value: Template) -> MessageActionBuilder {
        self.values.register_template(key.as_ref(), value);
        self
    }

    pub fn inject_mode(mut self, mode: InjectMode) -> MessageActionBuilder {
        self.inject_mode = mode;
        self
    }

    pub fn build(self) -> MessageAction {
        MessageAction {
            uuid: self.uuid,
            name: self.name,
            message: self.message,
            values: self.values,
            when: self.when,
            inject_mode: self.inject_mode,
        }
    }
}
