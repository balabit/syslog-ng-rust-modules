// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::VecDeque;
use std::time::Duration;

use uuid::Uuid;

use config::action::ActionType;
use conditions::Conditions;
use state::State;
use action::Action;
use Event;
use Template;
use Alert;

pub struct BaseContext<E, T> where E: Event, T: Template<Event=E> {
    pub name: Option<String>,
    pub uuid: Uuid,
    pub conditions: Conditions,
    pub actions: Vec<ActionType<T>>,
    pub patterns: Vec<String>,
}

impl<E, T> BaseContext<E, T> where E: Event, T: Template<Event=E> {
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn actions(&self) -> &[ActionType<T>] {
        &self.actions
    }

    pub fn is_opening(&self, message: &E) -> bool {
        if self.conditions.first_opens {
            self.patterns.first().iter().any(|first| message.ids().into_iter().any(|id| &id[..] == first.as_bytes()))
        } else {
            true
        }
    }

    pub fn is_closing(&self, state: &State<E>) -> bool {
        trace!("Conditions: shoud we close this context?");
        state.is_open() && self.is_closing_condition_met(state)
    }

    fn is_closing_condition_met(&self, state: &State<E>) -> bool {
        self.is_max_size_reached(state) || self.is_closing_message(state) ||
        self.is_any_timer_expired(state)
    }

    fn is_max_size_reached(&self, state: &State<E>) -> bool {
        self.conditions.max_size.map_or(false, |max_size| state.messages().len() >= max_size)
    }

    fn is_closing_message(&self, state: &State<E>) -> bool {
        if self.conditions.last_closes {
            state.messages().last().iter().any(|last_message| {
                self.patterns.last().iter().any(|last| last_message.ids().into_iter().any(|id| &id[..] == last.as_bytes()))
            })
        } else {
            false
        }
    }

    fn is_any_timer_expired(&self, state: &State<E>) -> bool {
        self.is_timeout_expired(state) || self.is_renew_timeout_expired(state)
    }

    fn is_timeout_expired(&self, state: &State<E>) -> bool {
        state.elapsed_time() >= self.conditions.timeout
    }

    fn is_renew_timeout_expired(&self, state: &State<E>) -> bool {
        self.conditions.renew_timeout.map_or(false, |renew_timeout| {
            state.elapsed_time_since_last_message() >= renew_timeout
        })
    }

    pub fn on_timer(&self,
                    event: &Duration,
                    state: &mut State<E>,
                    responder: &mut VecDeque<Alert<E>>) {
        if state.is_open() {
            state.update_timers(event);
        }
        if self.is_closing(state) {
            self.close(state, responder);
        }
    }

    pub fn on_message(&self,
                      event: E,
                      state: &mut State<E>,
                      responder: &mut VecDeque<Alert<E>>) {
        if state.is_open() {
            state.add_message(event);
        } else if self.is_opening(&event) {
            state.add_message(event);
            self.open(state, responder);
        }

        if self.is_closing(state) {
            self.close(state, responder);
        }
    }

    fn open(&self, state: &mut State<E>, responder: &mut VecDeque<Alert<E>>) {
        trace!("Context: opening state; uuid={}", self.uuid());
        for i in self.actions() {
            i.on_opened(state, self, responder);
        }
        state.open();
    }

    fn close(&self, state: &mut State<E>, responder: &mut VecDeque<Alert<E>>) {
        trace!("Context: closing state; uuid={}", self.uuid());
        for i in self.actions() {
            i.on_closed(state, self, responder);
        }
        state.close();
    }
}
