use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Message {
    uuid: PatternId,
    name: Option<PatternId>,
    data: BTreeMap<String, String>
}

impl Message {
    pub fn uuid(&self) -> &PatternId {
        &self.uuid
    }

    pub fn name(&self) -> Option<&PatternId> {
        self.name.as_ref()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

pub struct Builder {
    uuid: PatternId,
    name: Option<PatternId>,
    data: BTreeMap<String, String>
}

impl Builder {
    pub fn new(uuid: &str) -> Builder {
        Builder {
            uuid: PatternId::Uuid(uuid.to_string()),
            name: None,
            data: BTreeMap::new()
        }
    }

    pub fn name(&mut self, name: String) -> &mut Builder {
        self.name = Some(PatternId::Name(name));
        self
    }

    pub fn pair(&mut self, key: String, value: String) -> &mut Builder {
        self.data.insert(key, value);
        self
    }

    pub fn build(&self) -> Message {
        Message {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            data: self.data.clone()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PatternId {
    Uuid(String),
    Name(String)
}

mod deser {
    use super::PatternId;
    use serde;
    use serde::de::Deserialize;

    const VARIANTS: &'static [&'static str] = &["uuid", "name"];

    impl serde::Deserialize for PatternId {
        fn deserialize<D>(deserializer: &mut D) -> Result<PatternId, D::Error>
            where D: serde::de::Deserializer
        {
            deserializer.visit_enum("PatternId", VARIANTS, PatternIdVisitor)
        }
    }

    enum Field {
        Uuid,
        Name,
    }

    impl serde::Deserialize for Field {
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
                        "uuid" => Ok(Field::Uuid),
                        "name" => Ok(Field::Name),
                        name @ _ => Err(serde::de::Error::unknown_field(name)),
                    }
                }
            }

            deserializer.visit(FieldVisitor)
        }
    }

    struct PatternIdVisitor;

    impl serde::de::EnumVisitor for PatternIdVisitor {
        type Value = PatternId;

        fn visit<V>(&mut self, mut visitor: V) -> Result<PatternId, V::Error>
            where V: serde::de::VariantVisitor
        {
            match try!(visitor.visit_variant()) {
                Field::Uuid => {
                    let value = try!(visitor.visit_newtype());
                    Ok(PatternId::Uuid(value))
                },
                Field::Name => {
                    let value = try!(visitor.visit_newtype());
                    Ok(PatternId::Name(value))
                },
            }
        }
    }
}
