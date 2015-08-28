use conditions::Builder;
use Context;
use TimerEvent;
use message;

use uuid::Uuid;
use std::rc::Rc;

#[test]
fn test_given_map_context_when_messages_have_the_same_kvpairs_then_they_go_to_the_same_context() {
    let delta = 10;
    let timeout = 30;
    let event = TimerEvent(delta);
    let msg_id1 = "11eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
    let msg_id2 = "21eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
    let msg_id3 = "31eaf6f8-0640-460f-aee2-a72d2f2ab258".to_string();
    let patterns = vec![
        msg_id1.clone(),
        msg_id2.clone(),
        msg_id3.clone(),
    ];
    let mut context = Context::new_map(Uuid::new_v4(), Builder::new(timeout).patterns(patterns).build());
    let msg1 = message::Builder::new(&msg_id1)
                                .pair("HOST".to_string(), "host".to_string())
                                .pair("PROGRAM".to_string(), "program".to_string())
                                .pair("PID".to_string(), "pid".to_string())
                                .build();
    let msg2 = message::Builder::new(&msg_id2)
                                .pair("HOST".to_string(), "host2".to_string())
                                .pair("PROGRAM".to_string(), "program2".to_string())
                                .pair("PID".to_string(), "pid2".to_string())
                                .build();
    let msg3 = message::Builder::new(&msg_id3)
                                .pair("HOST".to_string(), "host".to_string())
                                .pair("PROGRAM".to_string(), "program".to_string())
                                .pair("PID".to_string(), "pid".to_string())
                                .build();

    assert_false!(context.is_open());
    context.on_message(Rc::new(msg1));
    assert_true!(context.is_open());
    context.on_timer(&event);
    context.on_message(Rc::new(msg2));
    context.on_message(Rc::new(msg3));
    context.on_timer(&event);
    context.on_timer(&event);
    assert_true!(context.is_open());
    context.on_timer(&event);
    assert_false!(context.is_open());
}
