extern crate correlation_parser;

use correlation_parser::{Timer, Watchdog};
use correlation_parser::mock::{MockEvent, MockLogTemplate};

use std::time::Duration;
use std::sync::{Arc, Mutex};

fn create_counter_and_timer(cb_interval: Duration) -> (Arc<Mutex<u32>>, Watchdog) {
    let counter = Arc::new(Mutex::new(0));
    let cloned_counter = counter.clone();

    let timer = Watchdog::schedule(cb_interval, move || {
        if let Ok(mut guard) = cloned_counter.lock() {
            *guard += 1;
        }
    });

    (counter, timer)
}

fn assert_callback_called(cb_interval: Duration, iter_count: u32, acceptable_bias: u32) {
    let (counter, timer) = create_counter_and_timer(cb_interval);

    Timer::<MockEvent, MockLogTemplate>::start(&timer);
    ::std::thread::sleep(cb_interval * iter_count);
    Timer::<MockEvent, MockLogTemplate>::stop(&timer);

    if let Ok(guard) = counter.lock() {
        let less = *guard < (iter_count - acceptable_bias);
        let greater = *guard > (iter_count + acceptable_bias);
        let in_middle = !less && !greater;
        assert!(in_middle);
    } else {
        unreachable!();
    };
}

#[test]
fn test_watchdog_calls_the_provided_callback_after_it_is_started() {
    assert_callback_called(Duration::from_millis(50), 6, 1);
    assert_callback_called(Duration::from_millis(50), 60, 5);
}
