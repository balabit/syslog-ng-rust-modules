use syslog_ng_common::{LogMessage, GlobalConfig};
use correlation::{Message, Event, EventIds, Template, TemplateFactory, CompileError};

use std::borrow::Borrow;
use std::fmt::Write;

#[derive(Clone)]
pub struct MockEvent(pub Message);

impl Event for MockEvent {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key)
    }
    fn uuid(&self) -> &str {
        self.0.uuid()
    }
    fn ids(&self) -> EventIds {
        self.0.ids()
    }
    fn new(uuid: &str, message: &str) -> Self {
        MockEvent(Message::new(uuid, message))
    }
    fn set_name(&mut self, name: Option<&str>) {
        self.0.set_name(name);
    }
    fn name(&self) -> Option<&str> {
        self.0.name()
    }
    fn set(&mut self, key: &str, value: &str) {
        self.0.set(key, value);
    }
    fn set_message(&mut self, message: &str) {
        self.0.set_message(message);
    }
    fn message(&self) -> &str {
        self.0.message()
    }
}

impl Into<LogMessage> for MockEvent {
    fn into(self) -> LogMessage {
        let mut logmsg = LogMessage::new();
        for (k, v) in self.0.values.iter() {
            logmsg.insert(k.borrow(), v.borrow());
        }
        logmsg.insert("MESSAGE", &self.0.message);
        logmsg
    }
}

pub struct MockLogTemplate(String);

impl Template for MockLogTemplate {
    type Event = MockEvent;
    fn format_with_context(&self, _: &[Self::Event], _: &str, buffer: &mut String) {
        let _ = buffer.write_str(&self.0);
    }
}

pub struct MockLogTemplateFactory;

impl TemplateFactory<MockEvent> for MockLogTemplateFactory {
    type Template = MockLogTemplate;
    fn compile(&self, value: &str) -> Result<Self::Template, CompileError> {
        Ok(MockLogTemplate(value.to_owned()))
    }
}

impl From<GlobalConfig> for MockLogTemplateFactory {
    fn from(_: GlobalConfig) -> MockLogTemplateFactory {
        MockLogTemplateFactory
    }
}


use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use correlation::correlator::Correlator;
use Timer;

pub struct MockTimer<E, T> where E: 'static + Event, T: 'static + Template<Event=E> {
    correlator: Arc<Mutex<Correlator<E, T>>>,
    delta: Duration,
}

impl<E, T> MockTimer<E, T> where E: 'static + Event, T: 'static + Template<Event=E> {
    pub fn elapse_time(&mut self, delta: Duration) {
        let mut guard = self.correlator.lock().unwrap();
        guard.elapse_time(delta);
    }

    pub fn elapse_set_time(&mut self) {
        let mut guard = self.correlator.lock().unwrap();
        guard.elapse_time(self.delta);
    }
}

impl<E, T> Clone for MockTimer<E, T> where E: 'static + Event, T: 'static + Template<Event=E> {
    fn clone(&self) -> MockTimer<E, T> {
        MockTimer {
            delta: self.delta,
            correlator: self.correlator.clone()
        }
    }
}

impl<E, T> Timer<E, T> for MockTimer<E, T> where E: 'static + Event + Send, T: 'static + Template<Event=E> {
    fn new(delta: Duration, correlator: Arc<Mutex<Correlator<E, T>>>) -> Self {
        MockTimer {
            delta: delta,
            correlator: correlator
        }
    }
}
