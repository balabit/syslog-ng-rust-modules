#[macro_use]
extern crate maplit;
extern crate correlation;

use correlation::Correlator;
use std::thread;

fn main() {
    let mut correlator = Correlator::new();
    let msg1 = btreemap!{
        "uuid".to_string() => "1".to_string(),
    };
    correlator.push_message(msg1);
    thread::sleep_ms(2000);
    correlator.stop();
}
