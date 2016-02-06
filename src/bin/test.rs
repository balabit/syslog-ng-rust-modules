#[macro_use]
extern crate maplit;
extern crate correlation;
extern crate uuid;

use correlation::correlator::Correlator;
use correlation::config::ContextConfigBuilder;
use correlation::ConditionsBuilder;
use correlation::MessageBuilder;
use correlation::ContextMap;
use uuid::Uuid;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn main() {
    let uuid1 = "1b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let uuid2 = "2b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let uuid3 = "3b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_string();
    let patterns = vec![
        uuid1.clone(),
        uuid2.clone(),
        uuid3.clone(),
    ];
    let condition = ConditionsBuilder::new(Duration::from_millis(100))
                        .patterns(patterns)
                        .first_opens(true)
                        .last_closes(true)
                        .build();
    let contexts = vec![
        ContextConfigBuilder::new(Uuid::new_v4(), condition.clone()).actions(Vec::new()).build(),
        ContextConfigBuilder::new(Uuid::new_v4(), condition.clone()).actions(Vec::new()).build(),
        ContextConfigBuilder::new(Uuid::new_v4(), condition.clone()).actions(Vec::new()).build(),
    ];
    let mut correlator = Correlator::new(ContextMap::from_configs(contexts));
    let _ = correlator.push_message(MessageBuilder::new(&uuid1, "message").build());
    thread::sleep(Duration::from_millis(20));
    let _ = correlator.push_message(MessageBuilder::new(&uuid2, "message").build());
    thread::sleep(Duration::from_millis(80));
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message").build());
    let _ = correlator.stop();
}
