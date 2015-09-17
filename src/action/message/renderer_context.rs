use message::Message;
use uuid::Uuid;
use rustc_serialize::json::{
    Json,
    ToJson
};
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct RendererContext<'m, 'n, 'u> {
    messages: &'m Vec<Rc<Message>>,
    context_name: &'n String,
    context_uuid: &'u Uuid
}

impl<'m, 'n, 'u> ToJson for RendererContext<'m, 'n, 'u> {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("context.name".to_string(), self.context_name.to_json());
        m.insert("context.uuid".to_string(), self.context_uuid.to_hyphenated_string().to_json());
        m.insert("messages".to_string(), rc_message_to_json(self.messages));
        m.to_json()
    }
}

fn rc_message_to_json(messages: &[Rc<Message>]) -> Json {
    let mut array: Vec<&Message> = Vec::new();
    for i in messages {
        array.push(i);
    }
    array.to_json()
}
