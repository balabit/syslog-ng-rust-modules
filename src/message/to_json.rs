// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use super::Message;

use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;

impl<'a> ToJson for &'a Message {
    fn to_json(&self) -> Json {
        (*self).to_json()
    }
}

impl ToJson for Message {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("uuid".to_owned(), self.uuid.to_json());
        m.insert("name".to_owned(), self.name.to_json());
        m.insert("message".to_owned(), self.message.to_json());
        m.insert("values".to_owned(), self.values.to_json());
        m.to_json()
    }
}
