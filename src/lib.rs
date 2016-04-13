#[macro_use]
extern crate log;
#[macro_use]
extern crate syslog_ng_common;
extern crate correlation;

use correlation::{Request, Alert, Event, Template, TemplateFactory};
use correlation::config::action::message::InjectMode;
use correlation::correlator::{Correlator, AlertHandler, CorrelatorFactory};
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::sync::{mpsc, Arc, Mutex};
use syslog_ng_common::{MessageFormatter, LogMessage};
use syslog_ng_common::{Parser, ParserBuilder, OptionError, Pipe};

pub use logevent::LogEvent;
pub use logtemplate::{LogTemplate, LogTemplateFactory};

pub mod options;
pub mod logevent;
pub mod mock;
pub mod logtemplate;

pub const CLASSIFIER_UUID: &'static str = ".classifier.uuid";
pub const CLASSIFIER_CLASS: &'static str = ".classifier.class";

struct MessageSender;

impl<P, E> AlertHandler<P, E> for MessageSender where P: Pipe, E: Event + Into<LogMessage> {
    fn on_alert(&mut self, alert: Alert<E>, reactor_input_channel: &mut mpsc::Sender<Request<E>>, parent: &mut P) {
        match alert.inject_mode {
            InjectMode::Log => {
                debug!("LOG: {}", alert.message.message());
            },
            InjectMode::Forward => {
                debug!("FORWARD: {}", alert.message.message());
                let logmsg = alert.message.into();
                parent.forward(logmsg);
            },
            InjectMode::Loopback => {
                debug!("LOOPBACK: {}", alert.message.message());
                if let Err(err) = reactor_input_channel.send(Request::Message(Arc::new(alert.message))) {
                    error!("{}", err);
                }
            },
        }
    }
}

pub struct CorrelationParserBuilder<P, E, T, TF> where P: Pipe, E: 'static + Event, T: 'static + Template<Event=E>, TF: TemplateFactory<E, Template=T> {
    contexts: Option<Correlator<P, E, T>>,
    formatter: MessageFormatter,
    template_factory: TF,
    _marker: PhantomData<(P, E, T, TF)>
}

impl<P, E, T, TF> CorrelationParserBuilder<P, E, T, TF> where P: Pipe, E: Event, T: Template<Event=E>, TF: TemplateFactory<E, Template=T> {
    pub fn set_file(&mut self, path: &str) {
        match CorrelatorFactory::from_path(path, &self.template_factory) {
            Ok(correlator) => {
                self.contexts = Some(correlator);
            },
            Err(err) => {
                error!("CorrelationParser: failed to set config file: {:?}", &err);
            }
        }
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.formatter.set_prefix(prefix);
    }
}

impl<P, E, T, TF> ParserBuilder<P> for CorrelationParserBuilder<P, E, T, TF> where P: Pipe, E: 'static + Event + Into<LogMessage>, T: 'static + Template<Event=E>, TF: TemplateFactory<E, Template=T> + Default {
    type Parser = CorrelationParser<P, E, T>;
    fn new() -> Self {
        CorrelationParserBuilder {
            contexts: None,
            formatter: MessageFormatter::new(),
            template_factory: TF::default(),
            _marker: PhantomData
        }
    }
    fn option(&mut self, name: String, value: String) {
        debug!("CorrelationParser: set_option(key={}, value={})", &name, &value);

        match name.borrow() {
            "contexts_file" => self.set_file(&value),
            "prefix" => self.set_prefix(value),
            _ => debug!("CorrelationParser: not supported key: {:?}", name)
        };
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        debug!("Building CorrelationParser");
        let CorrelationParserBuilder {contexts, template_factory, formatter, _marker } = self;
        let mut contexts = try!(contexts.ok_or(OptionError::missing_required_option(options::CONTEXTS_FILE)));
        contexts.set_alert_handler(Some(Box::new(MessageSender)));
        Ok(CorrelationParser::new(contexts, formatter))
    }
}

pub struct CorrelationParser<P: Pipe, E: 'static + Event, T: 'static + Template<Event=E>> {
    correlator: Arc<Mutex<Correlator<P, E, T>>>,
    formatter: MessageFormatter,
}

impl<P, E, T> Clone for CorrelationParser<P, E, T> where P: Pipe, E: Event, T: Template<Event=E> {
    fn clone(&self) -> CorrelationParser<P, E, T> {
        CorrelationParser { correlator: self.correlator.clone(), formatter: self.formatter.clone() }
    }
}

impl<P, E, T> CorrelationParser<P, E, T> where P: Pipe, E: Event, T: Template<Event=E> {
    pub fn new(correlator: Correlator<P, E, T>, formatter: MessageFormatter) -> CorrelationParser<P, E, T> {
        CorrelationParser {
            correlator: Arc::new(Mutex::new(correlator)),
            formatter: formatter,
        }
    }
}

impl<P, E, T> Parser<P> for CorrelationParser<P, E, T> where P: Pipe, E: Event, T: Template<Event=E> {
    fn parse(&mut self, parent: &mut P, msg: &mut LogMessage, message: &str) -> bool {
        debug!("CorrelationParser: process()");
        let message = {
            if let Some(uuid) = msg.get(CLASSIFIER_UUID) {
                let name = msg.get(CLASSIFIER_CLASS);

                let mut event = E::new(uuid, message);
                for (k, v) in msg.values() {
                    event.set(&k, &v);
                }
                event.set_name(name);
                event
            } else {
                return false;
            }
        };

        match self.correlator.lock() {
            Ok(mut guard) => {
                guard.handle_events(parent);
                match guard.push_message(message) {
                    Ok(_) => true,
                    Err(err) => {
                        error!("{}", err);
                        false
                    }
                }
            },
            Err(err) => {
                error!("{}", err);
                false
            }
        }
    }
}

parser_plugin!(CorrelationParserBuilder<LogParser, LogEvent, LogTemplate, LogTemplateFactory>);
