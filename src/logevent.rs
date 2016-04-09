use syslog_ng_common::LogMessage;
use correlation::{Event, EventIds};

use super::CLASSIFIER_UUID;
use super::CLASSIFIER_CLASS;

#[derive(Clone)]
pub struct LogEvent(pub LogMessage);

unsafe impl Sync for LogEvent {}

impl Event for LogEvent {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key)
    }
    fn uuid(&self) -> &str {
        self.get(CLASSIFIER_UUID).unwrap()
    }
    fn ids(&self) -> EventIds {
        EventIds {
            uuid: self.uuid(),
            name: self.name()
        }
    }
    fn new(uuid: &str, message: &str) -> Self {
        let mut msg = LogMessage::new();
        msg.insert(CLASSIFIER_UUID, uuid);
        msg.insert("MESSAGE", message);
        LogEvent(msg)
    }
    fn set_name(&mut self, name: Option<&str>) {
        if let Some(name) = name {
            self.0.insert(CLASSIFIER_CLASS, name);
        }
    }
    fn name(&self) -> Option<&str> {
        self.0.get(CLASSIFIER_CLASS)
    }
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key, value);
    }
    fn set_message(&mut self, message: &str) {
        self.0.insert("MESSAGE", message);
    }
    fn message(&self) -> &str {
        self.0.get("MESSAGE").unwrap()
    }
}
