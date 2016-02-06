use action::Action;
use context::base::BaseContext;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use message::{Message, MessageBuilder};

use std::borrow::Borrow;
use std::collections::BTreeMap;
use state::State;
use self::error::Error;
use self::renderer_context::RendererContext;
use super::ExecCondition;
use handlebars::{Handlebars, Context, Template};

pub use self::builder::MessageActionBuilder;

mod error;
mod renderer_context;
mod deser;
mod builder;
#[cfg(test)]
mod test;

pub const CONTEXT_UUID: &'static str = "context_uuid";
pub const CONTEXT_NAME: &'static str = "context_name";
pub const CONTEXT_LEN: &'static str = "context_len";
pub const MESSAGES: &'static str = "messages";
const MESSAGE: &'static str = "MESSAGE";

pub struct MessageAction {
    uuid: String,
    name: Option<String>,
    message: Template,
    values: Handlebars,
    when: ExecCondition,
    inject_mode: InjectMode,
}

impl MessageAction {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn message(&self) -> &Template {
        &self.message
    }
    pub fn values(&self) -> &Handlebars {
        &self.values
    }
    pub fn inject_mode(&self) -> &InjectMode {
        &self.inject_mode
    }

    fn render_value(&self, key: &str, template_context: &Context) -> Result<String, Error> {
        let mut writer = Vec::new();
        {
            try!(self.values.renderw(key, &template_context, &mut writer));
        }
        let string = try!(String::from_utf8(writer));
        Ok(string)
    }

    fn render_values(&self, template_context: &Context) -> Result<BTreeMap<String, String>, Error> {
        let mut rendered_values = BTreeMap::new();
        for (key, _) in self.values.get_templates() {
            let rendered_value = try!(self.render_value(key, &template_context));
            rendered_values.insert(key.to_owned(), rendered_value);
        }
        Ok(rendered_values)
    }

    fn render_message(&self, state: &State, context: &BaseContext) -> Result<Message, Error> {
        let template_context = {
            use handlebars::Context;
            let context = RendererContext::new(state, context);
            Context::wraps(&context)
        };

        let mut rendered_values = try!(self.render_values(&template_context));
        MessageAction::extend_with_context_information(&mut rendered_values, state, context);
        let message = rendered_values.remove(MESSAGE)
                                     .expect(&format!("There is no '{}' key in the renderer \
                                                       key-value pairs",
                                                      MESSAGE));
        let name = self.name.as_ref().map(|name| name.borrow());
        let message = MessageBuilder::new(&self.uuid, message)
                          .name(name)
                          .values(rendered_values)
                          .build();
        Ok(message)
    }

    fn extend_with_context_information(values: &mut BTreeMap<String, String>,
                                       state: &State,
                                       context: &BaseContext) {
        values.insert(CONTEXT_UUID.to_owned(),
                      context.uuid().to_hyphenated_string());
        values.insert(CONTEXT_LEN.to_owned(), state.messages().len().to_string());
        if let Some(name) = context.name() {
            values.insert(CONTEXT_NAME.to_owned(), name.to_owned());
        }
    }

    fn execute(&self, state: &State, context: &BaseContext, responder: &mut ResponseSender) {
        match self.render_message(state, context) {
            Ok(message) => {
                let response = Alert {
                    message: message,
                    inject_mode: self.inject_mode.clone(),
                };
                responder.send_response(Response::Alert(response));
            }
            Err(error) => {
                error!("{}", error);
            }
        }
    }
}

impl From<MessageAction> for super::ActionType {
    fn from(action: MessageAction) -> super::ActionType {
        super::ActionType::Message(action)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InjectMode {
    Log,
    Forward,
    Loopback,
}

impl Default for InjectMode {
    fn default() -> InjectMode {
        InjectMode::Log
    }
}

#[derive(Debug, Clone)]
pub struct Alert {
    message: Message,
    inject_mode: InjectMode,
}

impl Alert {
    pub fn message(&self) -> &Message {
        &self.message
    }
}

impl Action for MessageAction {
    fn on_opened(&self, state: &State, context: &BaseContext, responder: &mut ResponseSender) {
        if self.when.on_opened {
            trace!("MessageAction: on_opened()");
            self.execute(state, context, responder);
        }
    }

    fn on_closed(&self, state: &State, context: &BaseContext, responder: &mut ResponseSender) {
        if self.when.on_closed {
            trace!("MessageAction: on_closed()");
            self.execute(state, context, responder);
        }
    }
}
