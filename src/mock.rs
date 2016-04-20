use syslog_ng_common::{LogMessage, GlobalConfig};
use correlation::{Message, Event, EventIds, Template, TemplateFactory, CompileError};

use std::io::Write;

#[derive(Clone)]
pub struct MockEvent(pub Message);

impl Event for MockEvent {
    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.0.get(key)
    }
    fn uuid(&self) -> &[u8] {
        self.0.uuid()
    }
    fn ids(&self) -> EventIds {
        self.0.ids()
    }
    fn new(uuid: &[u8], message: &[u8]) -> Self {
        MockEvent(Message::new(uuid, message))
    }
    fn set_name(&mut self, name: Option<&[u8]>) {
        self.0.set_name(name);
    }
    fn name(&self) -> Option<&[u8]> {
        self.0.name()
    }
    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.0.set(key, value);
    }
    fn set_message(&mut self, message: &[u8]) {
        self.0.set_message(message);
    }
    fn message(&self) -> &[u8] {
        self.0.message()
    }
}

impl Into<LogMessage> for MockEvent {
    fn into(self) -> LogMessage {
        let mut logmsg = LogMessage::new();
        for (k, v) in self.0.values.iter() {
            logmsg.insert(&k[..], &v[..]);
        }
        logmsg.insert("MESSAGE", &self.0.message);
        logmsg
    }
}

pub struct MockLogTemplate(String);

impl Template for MockLogTemplate {
    type Event = MockEvent;
    fn format_with_context(&self, _: &[Self::Event], _: &str, buffer: &mut Write) {
        let _ = buffer.write(self.0.as_bytes());
    }
}

pub struct MockLogTemplateFactory;

impl TemplateFactory<MockEvent> for MockLogTemplateFactory {
    type Template = MockLogTemplate;
    fn compile(&self, value: &[u8]) -> Result<Self::Template, CompileError> {
        Ok(MockLogTemplate(String::from_utf8_lossy(value).to_string()))
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
