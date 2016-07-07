// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

error_chain! {
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    links {}

    foreign_links {}

    errors {
      MissingRequiredOption(option: String) {
        description("A required option is missing")
        display("A required option is missing: {}", option)
      }

    InvalidValue(option_name: String, value: String, expected_value: String) {
        description("Invalid value in option")
        display("Invalid value in option. option_name={} value={} expected_value={}", option_name, value, expected_value)
    }
   }
}

impl Error {
    pub fn missing_required_option<S: Into<String>>(option: S) -> Error {
        ErrorKind::MissingRequiredOption(option.into()).into()
    }
    pub fn invalid_value<S: Into<String>>(option_name: S, value: S, expected_value: S) -> Error {
        ErrorKind::InvalidValue(option_name.into(), value.into(), expected_value.into()).into()
    }
    pub fn verbatim_error<S: Into<String>>(error_msg: S) -> Error {
        ErrorKind::Msg(error_msg.into()).into()
    }
}
