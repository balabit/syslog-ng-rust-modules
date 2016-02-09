// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use message::Message;
use state::State;
use context::BaseContext;

use uuid::Uuid;
use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use std::sync::Arc;

use super::{CONTEXT_LEN, CONTEXT_NAME, CONTEXT_UUID, MESSAGES};

pub struct RendererContext<'m, 'c> {
    messages: &'m [Arc<Message>],
    context_name: Option<&'c String>,
    context_uuid: &'c Uuid,
}

impl<'m, 'c> RendererContext<'m, 'c> {
    pub fn new(state: &'m State, context: &'c BaseContext) -> RendererContext<'m, 'c> {
        RendererContext {
            messages: state.messages(),
            context_name: context.name(),
            context_uuid: context.uuid(),
        }
    }
}

impl<'m, 'c> ToJson for RendererContext<'m, 'c> {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        if let Some(name) = self.context_name {
            m.insert(CONTEXT_NAME.to_owned(), name.to_json());
        }
        m.insert(CONTEXT_UUID.to_owned(),
                 self.context_uuid.to_hyphenated_string().to_json());
        m.insert(CONTEXT_LEN.to_owned(), self.messages.len().to_json());
        m.insert(MESSAGES.to_owned(), rc_message_to_json(self.messages));
        m.to_json()
    }
}

fn rc_message_to_json(messages: &[Arc<Message>]) -> Json {
    let mut array: Vec<&Message> = Vec::new();
    for i in messages {
        array.push(i);
    }
    array.to_json()
}
