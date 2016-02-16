// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::sync::Arc;

use uuid::Uuid;

use config::action::ActionType;
use conditions::Conditions;
use state::State;
use dispatcher::response::ResponseSender;
use action::Action;
use message::Message;
use timer::TimerEvent;

pub struct BaseContext {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<ActionType>,
    pub patterns: Vec<String>,
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

    pub fn on_timer(&self,
                    event: &TimerEvent,
                    state: &mut State,
                    responder: &mut ResponseSender) {
        if state.is_open() {
            state.update_timers(event);
        }
        if self.conditions().is_closing(state) {
            self.close(state, responder);
        }
    }

    pub fn on_message(&self,
                      event: Arc<Message>,
                      state: &mut State,
                      responder: &mut ResponseSender) {
        if state.is_open() {
            state.add_message(event);
        } else if self.conditions().is_opening(&event) {
            state.add_message(event);
            self.open(state, responder);
        }

        if self.conditions().is_closing(state) {
            self.close(state, responder);
        }
    }

    fn open(&self, state: &mut State, responder: &mut ResponseSender) {
        trace!("Context: opening state; uuid={}", self.uuid());
        for i in self.actions() {
            i.on_opened(state, self, responder);
        }
        state.open();
    }

    fn close(&self, state: &mut State, responder: &mut ResponseSender) {
        trace!("Context: closing state; uuid={}", self.uuid());
        for i in self.actions() {
            i.on_closed(state, self, responder);
        }
        state.close();
    }
}

pub struct BaseContextBuilder {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    actions: Vec<ActionType>,
    patterns: Vec<String>
}

impl BaseContextBuilder {
    pub fn new(uuid: Uuid, conditions: Conditions) -> BaseContextBuilder {
        BaseContextBuilder {
            name: None,
            uuid: uuid,
            conditions: conditions,
            actions: Vec::new(),
            patterns: Vec::new()
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

    pub fn patterns(mut self, patterns: Vec<String>) -> BaseContextBuilder {
        self.patterns = patterns;
        self
    }
    pub fn build(self) -> BaseContext {
        let BaseContextBuilder {name, uuid, conditions, actions, patterns} = self;
        BaseContext {
            name: name,
            uuid: uuid,
            conditions: conditions,
            actions: actions,
            patterns: patterns
        }
    }
}
