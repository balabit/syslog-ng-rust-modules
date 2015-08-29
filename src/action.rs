use state::State;
use context::base::BaseContext;

pub use self::handlers::ActionHandlers;

#[derive(Clone, Debug)]
pub enum Action {
    Message(self::message::MessageAction)
}

impl Action {
    pub fn execute(&self, state: &State, context: &BaseContext) -> ExecResult {
        let result = match *self {
            Action::Message(ref action) => action.execute(state, context)
        };

        ExecResult::from(result)
    }
}

#[derive(Debug)]
pub enum ExecResult {
    Message(self::message::ExecResult)
}

mod deser {
    use serde;
    use super::Action;

    impl serde::de::Deserialize for Action {
    fn deserialize<D>(deserializer: &mut D) -> Result<Action, D::Error>
                      where D: serde::de::Deserializer {
        enum Field {
            Message,
        }

        impl serde::de::Deserialize for Field {
            #[inline]
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                where D: serde::de::Deserializer {
                    struct FieldVisitor;

                    impl serde::de::Visitor for FieldVisitor {
                        type Value = Field;

                        fn visit_str<E>(&mut self, value: &str) -> Result<Field, E> where E: serde::de::Error {
                            match value {
                                "message" => Ok(Field::Message),
                                _ => Err(serde::de::Error::unknown_field(value)),
                            }
                        }
                    }

                    deserializer.visit(FieldVisitor)
                }
            }

            struct Visitor;

            impl serde::de::EnumVisitor for Visitor {
                type Value = Action;

                fn visit<V>(&mut self, mut visitor: V) -> Result<Action, V::Error>
                    where V: serde::de::VariantVisitor
                {
                    match try!(visitor.visit_variant()) {
                        Field::Message => {
                            let value = try!(visitor.visit_newtype());
                            Ok(Action::Message(value))
                        }
                    }
                }
            }

            const VARIANTS: &'static [&'static str] = &["message"];

            deserializer.visit_enum("Action", VARIANTS, Visitor)
        }
    }

    #[cfg(test)]
    mod test {
        use serde_json::from_str;
        use action::Action;

        #[test]
        fn test_given_action_when_it_is_deserialized_then_we_get_the_right_result() {
            let text = r#"
                {
                    "message": null
                }
            "#;

            let result = from_str::<Action>(text);
            println!("{:?}", &result);
            let action = result.ok().expect("Failed to deserialize a valid Action");
            match action {
                Action::Message(_) => {}
            }
        }
    }
}

pub mod handlers {
    use super::ExecResult;
    use super::message;

    pub struct ActionHandlers {
        message_handler: Box<message::ActionHandler>
    }

    impl ActionHandlers {
        pub fn new(message: Box<message::ActionHandler>) -> ActionHandlers {
            ActionHandlers {
                message_handler: message
            }
        }

        pub fn handle(&mut self, command: ExecResult) {
            match command {
                ExecResult::Message(message) => self.message_handler.handle(message)
            }
        }
    }
}

pub mod message {
    use context::base::BaseContext;
    use state::State;
    use message::{Builder, Message};

    #[derive(Debug)]
    pub struct ExecResult(Message);

    impl From<ExecResult> for super::ExecResult {
        fn from(result: ExecResult) -> super::ExecResult {
            super::ExecResult::Message(result)
        }
    }

    #[derive(Clone, Debug)]
    pub struct MessageAction;

    impl MessageAction {
        pub fn new() -> MessageAction {
            MessageAction
        }

        pub fn execute(&self, _: &State, _: &BaseContext) -> ExecResult {
            ExecResult(Builder::new("8015340d-5b44-4b16-8a3f-60b505ccd22e").build())
        }
    }

    impl From<MessageAction> for super::Action {
        fn from(action: MessageAction) -> super::Action {
            super::Action::Message(action)
        }
    }

    pub trait ActionHandler {
        fn handle(&mut self, command: ExecResult);
    }

    mod deser {

    use super::MessageAction;
    use serde;
    use serde::de::Deserialize;

    impl serde::Deserialize for MessageAction {
        fn deserialize<D>(deserializer: &mut D) -> Result<MessageAction, D::Error>
            where D: serde::de::Deserializer
        {
            deserializer.visit_unit_struct("MessageAction", MessageActionVisitor)
        }
    }

    struct MessageActionVisitor;

    impl serde::de::Visitor for MessageActionVisitor {
        type Value = MessageAction;
        fn visit_unit<E>(&mut self) -> Result<Self::Value, E>
            where E: serde::de::Error {
            Ok(MessageAction)
        }
    }
    }
}
