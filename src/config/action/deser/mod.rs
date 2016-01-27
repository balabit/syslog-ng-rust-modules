use serde;
use super::{ActionType, ExecCondition, ON_CLOSED_DEFAULT, ON_OPENED_DEFAULT};

#[cfg(test)]
mod test;

impl serde::de::Deserialize for ActionType {
    fn deserialize<D>(deserializer: &mut D) -> Result<ActionType, D::Error>
        where D: serde::de::Deserializer
    {
        enum Field {
            Message,
        }

    impl serde::de::Deserialize for Field {
            #[inline]
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                where D: serde::de::Deserializer
            {
                struct FieldVisitor;

                impl serde::de::Visitor for FieldVisitor {
                    type Value = Field;

                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                        where E: serde::de::Error
                    {
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
            type Value = ActionType;

            fn visit<V>(&mut self, mut visitor: V) -> Result<ActionType, V::Error>
                where V: serde::de::VariantVisitor
            {
                match try!(visitor.visit_variant()) {
                    Field::Message => {
                        let value = try!(visitor.visit_newtype());
                        Ok(ActionType::Message(value))
                    }
                }
            }
        }

        const VARIANTS: &'static [&'static str] = &["message"];

        deserializer.visit_enum("ActionType", VARIANTS, Visitor)
    }
}

impl serde::de::Deserialize for ExecCondition {
    fn deserialize<D>(deserializer: &mut D) -> Result<ExecCondition, D::Error>
        where D: serde::de::Deserializer
    {
        enum Field {
            OnOpened,
            OnClosed,
        }

        impl serde::de::Deserialize for Field {
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                where D: serde::de::Deserializer
            {
                struct FieldVisitor;

                impl serde::de::Visitor for FieldVisitor {
                    type Value = Field;

                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                        where E: serde::de::Error
                    {
                        match value {
                            "on_opened" => Ok(Field::OnOpened),
                            "on_closed" => Ok(Field::OnClosed),
                            _ => Err(serde::de::Error::syntax(&format!("Unexpected field: {}", value))),
                        }
                    }
                }

                deserializer.visit(FieldVisitor)
            }
        }

        struct ExecConditionVisitor;

        impl serde::de::Visitor for ExecConditionVisitor {
            type Value = ExecCondition;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<ExecCondition, V::Error>
                where V: serde::de::MapVisitor
            {
                let mut on_opened: Option<bool> = ON_OPENED_DEFAULT;
                let mut on_closed: Option<bool> = ON_CLOSED_DEFAULT;

                while let Some(field) = try!(visitor.visit_key()) {
                    match field {
                        Field::OnOpened => on_opened = Some(try!(visitor.visit_value())),
                        Field::OnClosed => on_closed = Some(try!(visitor.visit_value()))
                    }
                }

                try!(visitor.end());

                Ok(ExecCondition{on_opened: on_opened, on_closed: on_closed})
            }
        }
        deserializer.visit_struct("ExecCondition", &[], ExecConditionVisitor)
    }
}
