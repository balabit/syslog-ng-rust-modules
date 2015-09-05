use serde;
use super::ActionType;

#[cfg(test)]
mod test;

impl serde::de::Deserialize for ActionType {
fn deserialize<D>(deserializer: &mut D) -> Result<ActionType, D::Error>
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
