use handlebars::Template;
use uuid::Uuid;

use config::action::ActionType;
use conditions::Conditions;

mod deser;
pub mod action;

pub struct Context {
    pub name: Option<String>,
    pub uuid: Uuid,
    pub conditions: Conditions,
    pub context_id: Option<Template>,
    pub actions: Vec<ActionType>,
}

pub struct ContextBuilder {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    context_id: Option<Template>,
    actions: Vec<ActionType>,
}

impl ContextBuilder {
    pub fn new(uuid: Uuid, conditions: Conditions) -> ContextBuilder {
        ContextBuilder {
            name: None,
            uuid: uuid,
            conditions: conditions,
            context_id: None,
            actions: Vec::new(),
        }
    }

    pub fn context_id(mut self, context_id: Option<Template>) -> ContextBuilder {
        self.context_id = context_id;
        self
    }

    pub fn actions(mut self, actions: Vec<ActionType>) -> ContextBuilder {
        self.actions = actions;
        self
    }

    pub fn name(mut self, name: String) -> ContextBuilder {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Context {
        Context {
            name: self.name,
            uuid: self.uuid,
            conditions: self.conditions,
            context_id: self.context_id,
            actions: self.actions,
        }
    }
}
