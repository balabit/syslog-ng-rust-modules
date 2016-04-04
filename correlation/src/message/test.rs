// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use message::MessageBuilder;

#[test]
fn test_given_message_when_it_does_not_have_a_name_then_its_ids_contain_only_the_its_uuid() {
    let uuid = "uuid1";
    let message = MessageBuilder::new(uuid, "message").build();
    assert_eq!(true, message.ids().any(|x| x == uuid));
    assert_eq!(1, message.ids().count());
}

#[test]
fn test_given_message_when_it_has_a_name_then_its_ids_contain_both_its_uuid_and_name() {
    let uuid = "uuid1";
    let name = "name";
    let message = MessageBuilder::new(uuid, "message").name(Some(name)).build();
    assert_eq!(true, message.ids().any(|x| x == name));
    assert_eq!(2, message.ids().count());
}
