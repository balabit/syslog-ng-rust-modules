// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use conditions::ConditionsBuilder;
use context::MapContext;
use message::MessageBuilder;
use test_utils::{MockTemplate, BaseContextBuilder};
use Message;

use uuid::Uuid;
use std::time::Duration;
use std::collections::VecDeque;

#[test]
fn test_given_map_context_when_messages_have_the_same_kvpairs_then_they_go_to_the_same_context() {
    let mut responder = VecDeque::default();
    let delta = Duration::from_millis(10);
    let timeout = Duration::from_millis(30);
    let msg_id1 = "11eaf6f8-0640-460f-aee2-a72d2f2ab258";
    let msg_id2 = "21eaf6f8-0640-460f-aee2-a72d2f2ab258";
    let msg_id3 = "31eaf6f8-0640-460f-aee2-a72d2f2ab258";
    let mut context = {
        let base_context = {
            let patterns = vec![
                msg_id1.to_owned(),
                msg_id2.to_owned(),
                msg_id3.to_owned(),
            ];
            let uuid = Uuid::new_v4();
            let conditions = ConditionsBuilder::new(timeout).build();
            BaseContextBuilder::<Message, MockTemplate>::new(uuid, conditions).patterns(patterns).build()
        };
        let context_key = ["HOST", "PROGRAM", "PID"].iter().map(|x| (*x).to_owned()).collect::<Vec<String>>();
        MapContext::new(base_context, context_key)
    };
    let msg1 = MessageBuilder::new(msg_id1, "message")
                   .pair(b"HOST", b"host")
                   .pair(b"PROGRAM", b"program")
                   .pair(b"PID", b"pid")
                   .build();
    let msg2 = MessageBuilder::new(msg_id2, "message")
                   .pair(b"HOST", b"host2")
                   .pair(b"PROGRAM", b"program2")
                   .pair(b"PID", b"pid2")
                   .build();
    let msg3 = MessageBuilder::new(msg_id3, "message")
                   .pair(b"HOST", b"host")
                   .pair(b"PROGRAM", b"program")
                   .pair(b"PID", b"pid")
                   .build();

    assert_false!(context.is_open());
    context.on_message(msg1, &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&delta, &mut responder);
    context.on_message(msg2, &mut responder);
    context.on_message(msg3, &mut responder);
    context.on_timer(&delta, &mut responder);
    context.on_timer(&delta, &mut responder);
    assert_true!(context.is_open());
    context.on_timer(&delta, &mut responder);
    assert_false!(context.is_open());
}
