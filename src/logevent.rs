use syslog_ng_common::LogMessage;
use correlation::{Event, EventIds};

use super::CLASSIFIER_UUID;
use super::CLASSIFIER_CLASS;

#[derive(Clone)]
pub struct LogEvent(pub LogMessage);

unsafe impl Sync for LogEvent {}

impl Event for LogEvent {
    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.0.get(key)
    }
    fn uuid(&self) -> &[u8] {
        self.get(CLASSIFIER_UUID).unwrap()
    }
    fn ids(&self) -> EventIds {
        EventIds {
            uuid: self.uuid(),
            name: self.name()
        }
    }
    fn new(uuid: &[u8], message: &[u8]) -> Self {
        let mut msg = LogMessage::new();
        msg.insert(CLASSIFIER_UUID, uuid);
        msg.insert("MESSAGE", message);
        LogEvent(msg)
    }
    fn set_name(&mut self, name: Option<&[u8]>) {
        if let Some(name) = name {
            self.0.insert(CLASSIFIER_CLASS, name);
        }
    }
    fn name(&self) -> Option<&[u8]> {
        self.0.get(CLASSIFIER_CLASS)
    }
    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.0.insert(key, value);
    }
    fn set_message(&mut self, message: &[u8]) {
        self.0.insert("MESSAGE", message);
    }
    fn message(&self) -> &[u8] {
        self.0.get("MESSAGE").unwrap()
    }
}

impl Into<LogMessage> for LogEvent {
    fn into(self) -> LogMessage {
        self.0
    }
}
