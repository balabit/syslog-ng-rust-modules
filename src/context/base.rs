use uuid::Uuid;
use std::rc::Rc;

use action::Action;
use conditions::Conditions;
use message::Message;
use state::State;
use timer::TimerEvent;

pub struct BaseContext {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<Box<Action>>
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

    pub fn on_timer(&self, event: &TimerEvent, state: &mut State) {
        if state.is_open() {
            state.update_timers(event);
            if self.conditions.is_closing(state) {
                self.close_state(state);
            }
        }
    }

    pub fn on_message(&self, event: Rc<Message>, state: &mut State) {
        if state.is_open() {
            state.add_message(event);
            if self.name.is_none() {
                println!("{} opened", &self.uuid.to_hyphenated_string());
            }
            if self.conditions.is_closing(state) {
                self.close_state(state);
            }
        } else if self.conditions.is_opening(&event) {
            println!("{:?} opening state", &self.name);
            if self.name.is_none() {
                println!("{} opening state", &self.uuid.to_hyphenated_string());
            }
            state.add_message(event);
            state.open();
            println!("{:?} state opened", &self.name);
        } else {
            println!("{:?} state DUNNO", &self.name);
            println!("Conditions {:?}", &self.conditions);
        }
    }

    fn close_state(&self, state: &mut State) {
        println!("{:?} closing state", &self.name);
        for i in &self.actions {
            i.execute(state, self);
        }
        state.close();
        println!("{:?} state closed", &self.name);
    }
}

pub struct BaseContextBuilder {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<Box<Action>>
}

impl BaseContextBuilder {
    pub fn new(uuid: Uuid, conditions: Conditions) -> BaseContextBuilder {
        BaseContextBuilder {
            name: None,
            uuid: uuid,
            conditions: conditions,
            actions: Vec::new()
        }
    }

    pub fn name(mut self, name: Option<String>) -> BaseContextBuilder {
        self.name = name;
        self
    }

    pub fn actions(mut self, actions: Vec<Box<Action>>) -> BaseContextBuilder {
        self.actions = actions;
        self
    }

    pub fn build(self) -> BaseContext {
        let BaseContextBuilder {name, uuid, conditions, actions} = self;
        BaseContext {
            name: name,
            uuid: uuid,
            conditions: conditions,
            actions: actions
        }
    }
}
