use std::time::Duration;
use std::str::FromStr;
use std::error::Error;
use serde::de;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SerializableDuration(pub Duration);

pub struct Visitor;

impl de::Visitor for Visitor {
    type Value = SerializableDuration;

    fn visit_u64<E>(&mut self, v: u64) -> Result<SerializableDuration, E>
        where E: de::Error
    {
        Ok(SerializableDuration(Duration::from_millis(v)))
    }

    fn visit_str<E>(&mut self, s: &str) -> Result<SerializableDuration, E>
        where E: de::Error
    {
        match FromStr::from_str(s) {
            Ok(value) => Ok(SerializableDuration(Duration::from_millis(value))),
            Err(error) => Err(E::syntax(error.description())),
        }
    }
}

impl de::Deserialize for SerializableDuration {
    fn deserialize<D>(deserializer: &mut D) -> Result<SerializableDuration, D::Error>
        where D: de::Deserializer
    {
        deserializer.visit_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::time::Duration;

    fn assert_serialized_value_eq(input: &str, expected: Duration) {
        let result = serde_json::from_str::<SerializableDuration>(input);
        println!("{:?}", &result);
        let duration = result.expect("Failed to deserialize a valid SerializableDuration value");
        assert_eq!(expected, duration.0);
    }

    #[test]
    fn test_given_valid_duration_when_it_is_deserialized_then_we_get_the_right_result() {
        assert_serialized_value_eq("100", Duration::from_millis(100));
    }

    #[test]
    fn test_given_valid_duration_as_str_when_it_is_deserialized_then_we_get_the_right_result() {
        assert_serialized_value_eq(r#""100""#, Duration::from_millis(100));
    }

    #[test]
    fn test_given_invalid_duration_when_it_is_deserialized_then_we_get_error() {
        let result = serde_json::from_str::<SerializableDuration>("word");
        println!("{:?}", &result);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn test_given_a_word_starting_with_numbers_when_it_is_deserialized_then_we_get_error() {
        let result = serde_json::from_str::<SerializableDuration>(r#""42word""#);
        println!("{:?}", &result);
        assert_eq!(true, result.is_err());
    }
}
