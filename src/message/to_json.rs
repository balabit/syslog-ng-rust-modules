use super::Message;

use rustc_serialize::json::{
    Json,
    ToJson
};
use std::collections::BTreeMap;
use std::rc::Rc;

impl<'a> ToJson for &'a Message {
    fn to_json(&self) -> Json {
        (*self).to_json()
    }
}

impl ToJson for Message {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("uuid".to_string(), self.uuid.to_json());
        m.insert("name".to_string(), self.name.to_json());
        m.insert("message".to_string(), self.message.to_json());
        m.insert("values".to_string(), self.values.to_json());
        m.to_json()
    }
}
