// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    MissingRequiredOption(String),
    InvalidValue {
        option_name: String,
        value: String,
        expected_value: String,
    },
    VerbatimError(String)
}

impl Error {
    pub fn missing_required_option<S: Into<String>>(option_name: S) -> Error {
        Error::MissingRequiredOption(option_name.into())
    }

    pub fn invalid_value<S: Into<String>>(option_name: S,
                                          value: S,
                                          expected_value: S)
                                          -> Error {
        Error::InvalidValue {
            option_name: option_name.into(),
            value: value.into(),
            expected_value: expected_value.into(),
        }
    }
    pub fn verbatim_error<S: Into<String>>(error_msg: S) -> Error {
        Error::VerbatimError(error_msg.into())
    }
}

use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            Error::MissingRequiredOption(ref name) => {
                formatter.write_fmt(format_args!("At least one required option is missing. \
                                                  option_name={}",
                                                 name))
            }
            Error::InvalidValue{ref option_name, ref value, ref expected_value} => {
                formatter.write_fmt(format_args!("Invalid value in option. option_name={} \
                                                  value={} expected_value={}",
                                                 option_name,
                                                 value,
                                                 expected_value))
            },
            Error::VerbatimError(ref error_msg) => formatter.write_str(error_msg)
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MissingRequiredOption(_) => "At least one required option is missing.",
            Error::InvalidValue{..} => "Invalid value in option.",
            Error::VerbatimError(..) => "Invalid value in option.",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::MissingRequiredOption(_) => None,
            Error::InvalidValue{..} => None,
            Error::VerbatimError(..) => None,
        }
    }
}
