use uuid::Uuid;

use config::action::ActionType;
use conditions::Conditions;

pub struct BaseContext {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<ActionType>,
}

impl BaseContext {
    pub fn conditions(&self) -> &Conditions {
        &self.conditions
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn actions(&self) -> &[ActionType] {
        &self.actions
    }
}

pub struct BaseContextBuilder {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<ActionType>,
}

impl BaseContextBuilder {
    pub fn new(uuid: Uuid, conditions: Conditions) -> BaseContextBuilder {
        BaseContextBuilder {
            name: None,
            uuid: uuid,
            conditions: conditions,
            actions: Vec::new(),
        }
    }

    pub fn name(mut self, name: Option<String>) -> BaseContextBuilder {
        self.name = name;
        self
    }

    pub fn actions(mut self, actions: Vec<ActionType>) -> BaseContextBuilder {
        self.actions = actions;
        self
    }

    pub fn build(self) -> BaseContext {
        let BaseContextBuilder {name, uuid, conditions, actions} = self;
        BaseContext {
            name: name,
            uuid: uuid,
            conditions: conditions,
            actions: actions,
        }
    }
}
