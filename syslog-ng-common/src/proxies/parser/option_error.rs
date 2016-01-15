#[derive(Debug)]
pub enum OptionError {
    MissingRequiredOption(String),
    InvalidValue {
        option_name: String,
        value: String,
        expected_value: String,
    },
}

impl OptionError {
    pub fn missing_required_option<S: Into<String>>(option_name: S) -> OptionError {
        OptionError::MissingRequiredOption(option_name.into())
    }

    pub fn invalid_value<S: Into<String>>(option_name: S,
                                          value: S,
                                          expected_value: S)
                                          -> OptionError {
        OptionError::InvalidValue {
            option_name: option_name.into(),
            value: value.into(),
            expected_value: expected_value.into(),
        }
    }
}

use std::fmt::{Display, Error, Formatter};

impl Display for OptionError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match *self {
            OptionError::MissingRequiredOption(ref name) => {
                formatter.write_fmt(format_args!("At least one required option is missing. \
                                                  option_name={}",
                                                 name))
            }
            OptionError::InvalidValue{ref option_name, ref value, ref expected_value} => {
                formatter.write_fmt(format_args!("Invalid value in option. option_name={} \
                                                  value={} expected_value={}",
                                                 option_name,
                                                 value,
                                                 expected_value))
            }
        }
    }
}

impl ::std::error::Error for OptionError {
    fn description(&self) -> &str {
        match *self {
            OptionError::MissingRequiredOption(_) => "At least one required option is missing.",
            OptionError::InvalidValue{..} => "Invalid value in option.",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            OptionError::MissingRequiredOption(_) => None,
            OptionError::InvalidValue{..} => None,
        }
    }
}
