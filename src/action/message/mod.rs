use action::Action;
use config;
use config::action::message::InjectMode;
use context::base::BaseContext;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use message::{Message, MessageBuilder};

use handlebars::Context;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use state::State;
use self::error::Error;
use self::renderer_context::RendererContext;

mod error;
mod renderer_context;
#[cfg(test)]
mod test;

pub const CONTEXT_UUID: &'static str = "context_uuid";
pub const CONTEXT_NAME: &'static str = "context_name";
pub const CONTEXT_LEN: &'static str = "context_len";
pub const MESSAGES: &'static str = "messages";
const MESSAGE: &'static str = "MESSAGE";

pub struct MessageAction {
    sender: Box<ResponseSender>,
    action: config::action::MessageAction,
}

impl MessageAction {
    pub fn new(sender: Box<ResponseSender>,
               mut action: config::action::MessageAction)
               -> MessageAction {
        let message = action.message.clone();
        action.values.register_template(MESSAGE, message);

        MessageAction {
            sender: sender,
            action: action,
        }
    }

    fn render_value(&self, key: &String, template_context: &Context) -> Result<String, Error> {
        let mut writer = Vec::new();
        {
            try!(self.action.values.renderw(key, &template_context, &mut writer));
        }
        let string = try!(String::from_utf8(writer));
        Ok(string)
    }

    fn render_values(&self, template_context: &Context) -> Result<BTreeMap<String, String>, Error> {
        let mut rendered_values = BTreeMap::new();
        for (key, _) in self.action.values.get_templates() {
            let rendered_value = try!(self.render_value(key, &template_context));
            rendered_values.insert(key.to_string(), rendered_value);
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
        let name = self.action.name.as_ref().map(|name| name.borrow());
        let message = MessageBuilder::new(&self.action.uuid, message)
                          .name(name)
                          .values(rendered_values)
                          .build();
        Ok(message)
    }

    fn extend_with_context_information(values: &mut BTreeMap<String, String>,
                                       state: &State,
                                       context: &BaseContext) {
        values.insert(CONTEXT_UUID.to_string(),
                      context.uuid().to_hyphenated_string());
        values.insert(CONTEXT_LEN.to_string(), state.messages().len().to_string());
        if let Some(name) = context.name() {
            values.insert(CONTEXT_NAME.to_string(), name.to_string());
        }
    }

    fn execute(&self, _state: &State, _context: &BaseContext, responder: &mut ResponseSender) {
        match self.render_message(_state, _context) {
            Ok(message) => {
                let response = Alert {
                    message: message,
                    inject_mode: self.action.inject_mode.clone(),
                };
                self.sender.send_response(Response::Alert(response));
            }
            Err(error) => {
                error!("{}", error);
            }
        }
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
    fn on_opened(&self, _state: &State, _context: &BaseContext, responder: &mut ResponseSender) {
        if self.action.when.on_opened {
            trace!("MessageAction: on_opened()");
            self.execute(_state, _context, responder);
        }
    }

    fn on_closed(&self, _state: &State, _context: &BaseContext, responder: &mut ResponseSender) {
        if self.action.when.on_closed {
            trace!("MessageAction: on_closed()");
            self.execute(_state, _context, responder);
        }
    }
}
