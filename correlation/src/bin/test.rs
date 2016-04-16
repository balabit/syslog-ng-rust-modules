// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate maplit;
extern crate correlation;
extern crate uuid;

use correlation::correlator::Correlator;
use correlation::config::ContextConfigBuilder;
use correlation::ConditionsBuilder;
use correlation::{MessageBuilder, Message};
use correlation::ContextMap;
use correlation::test_utils::{MockTemplate};
use uuid::Uuid;
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
                        .first_opens(true)
                        .last_closes(true)
                        .build();
    let contexts = vec![
        ContextConfigBuilder::new(Uuid::new_v4(), condition.clone()).patterns(patterns.clone()).actions(Vec::new()).build(),
        ContextConfigBuilder::new(Uuid::new_v4(), condition.clone()).patterns(patterns.clone()).actions(Vec::new()).build(),
        ContextConfigBuilder::new(Uuid::new_v4(), condition.clone()).patterns(patterns.clone()).actions(Vec::new()).build(),
    ];
    let mut correlator: Correlator<Message, MockTemplate> = Correlator::new(ContextMap::from_configs(contexts));
    correlator.push_message(MessageBuilder::new(&uuid1, "message").build());
    correlator.elapse_time(Duration::from_millis(20));
    correlator.push_message(MessageBuilder::new(&uuid2, "message").build());
    correlator.elapse_time(Duration::from_millis(80));
    correlator.push_message(MessageBuilder::new(&uuid3, "message").build());
}
