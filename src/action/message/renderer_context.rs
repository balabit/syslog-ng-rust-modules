use message::Message;
use state::State;
use context::BaseContext;

use uuid::Uuid;
use rustc_serialize::json::{
    Json,
    ToJson
};
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct RendererContext<'m, 'c> {
    messages: &'m [Rc<Message>],
    context_name: Option<&'c String>,
    context_uuid: &'c Uuid
}

impl<'m, 'c> RendererContext<'m, 'c> {
    pub fn new<'a>(state: &'m State, context: &'c BaseContext) -> RendererContext<'m, 'c> {
        RendererContext {
            messages: state.messages(),
            context_name: context.name(),
            context_uuid: context.uuid()
        }
    }
}

impl<'m, 'c> ToJson for RendererContext<'m, 'c> {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        if let Some(name) = self.context_name {
            m.insert("context.name".to_string(), name.to_json());
        }
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
