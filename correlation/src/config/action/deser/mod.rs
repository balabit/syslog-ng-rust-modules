use serde;
use super::{ActionType, ExecCondition};
use std::marker::PhantomData;

#[cfg(test)]
mod test;

impl<T> serde::de::Deserialize for ActionType<T> where T: serde::de::Deserialize {
    fn deserialize<D>(deserializer: &mut D) -> Result<ActionType<T>, D::Error>
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

                deserializer.deserialize(FieldVisitor)
            }
        }

        struct Visitor<T>(PhantomData<T>);

        impl<T> serde::de::EnumVisitor for Visitor<T> where T: serde::de::Deserialize {
            type Value = ActionType<T>;

            fn visit<V>(&mut self, mut visitor: V) -> Result<ActionType<T>, V::Error>
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

        deserializer.deserialize_enum("ActionType", VARIANTS, Visitor(PhantomData))
    }
}

impl serde::de::Deserialize for ExecCondition {
    fn deserialize<D>(deserializer: &mut D) -> Result<ExecCondition, D::Error>
        where D: serde::de::Deserializer
    {
        // the have the same On prefix
        #[allow(enum_variant_names)]
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
                            _ => {
                                Err(E::custom(format!("Unexpected field: {}",
                                                                      value)))
                            }
                        }
                    }
                }

                deserializer.deserialize(FieldVisitor)
            }
        }

        struct ExecConditionVisitor;

        impl serde::de::Visitor for ExecConditionVisitor {
            type Value = ExecCondition;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<ExecCondition, V::Error>
                where V: serde::de::MapVisitor
            {
                let mut condition: ExecCondition = Default::default();

                while let Some(field) = try!(visitor.visit_key()) {
                    match field {
                        Field::OnOpened => condition.on_opened = try!(visitor.visit_value()),
                        Field::OnClosed => condition.on_closed = try!(visitor.visit_value()),
                    }
                }

                try!(visitor.end());

                Ok(condition)
            }
        }
        deserializer.deserialize_struct("ExecCondition", &[], ExecConditionVisitor)
    }
}
