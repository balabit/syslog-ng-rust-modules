// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

quick_error! {
    /// Error type for configuration errors.
    ///
    /// This type is mainly used to indicate, that an `option(key, value)` was invalid (e.g.
    /// `value` is a path but the file doesn't exist).
    #[derive(Debug)]
    pub enum Error {
        /// A required option is missing.
        MissingRequiredOption(option: String) {
            description("A required option is missing")
            display("A required option is missing: {}", option)
        }
        /// `value` from `option(key, value)` is invalid (e.g. non-existing file).
        InvalidValue(option_name: String, value: String, expected_value: String) {
            description("Invalid value in option")
            display("Invalid value in option. option_name={} value={} expected_value={}", option_name, value, expected_value)
        }
        /// The specified configuration option is unknown. For example, you are only interested in
        // `option("regex", XXX)` values, and the user specified `option("foo", "bar")`.
        UnknownOption(option_name: String) {
            description("Unknown configuration option")
            display("Unknown configuration option: option_name={}", option_name)
        }
        /// Everything else.
        Verbatim(msg: String) {
            description(msg)
            display("{}", msg)
        }
    }
}

impl Error {
    /// Convenient constructor for `Error::MissingRequiredOption`
    ///
    /// # Examples
    ///
    /// ```
    /// use syslog_ng_common::Error;
    ///
    /// let _ = Error::missing_required_option("foo");
    /// let _ = Error::missing_required_option("bar".to_string());
    /// ```
    pub fn missing_required_option<S: Into<String>>(option: S) -> Error {
        Error::MissingRequiredOption(option.into()).into()
    }
    /// Convenient constructor for `Error::InvalidValue`
    ///
    /// # Examples
    ///
    /// ```
    /// use syslog_ng_common::Error;
    ///
    /// let _ = Error::invalid_value("option_name", "option_value", "URL");
    /// let _ = Error::invalid_value("option_name".to_string(), "option_value".to_string(), "URL".to_string());
    /// ```
    pub fn invalid_value<S: Into<String>>(option_name: S, value: S, expected_value: S) -> Error {
        Error::InvalidValue(option_name.into(), value.into(), expected_value.into()).into()
    }
    /// Convenient constructor for `Error::Verbatim`
    ///
    /// # Examples
    ///
    /// ```
    /// use syslog_ng_common::Error;
    ///
    /// let _ = Error::verbatim_error("error message");
    /// let _ = Error::verbatim_error("error message".to_string());
    /// ```
    pub fn verbatim_error<S: Into<String>>(error_msg: S) -> Error {
        Error::Verbatim(error_msg.into()).into()
    }
    /// Convenient constructor for `Error::UnknownOption`
    ///
    /// # Examples
    ///
    /// ```
    /// use syslog_ng_common::Error;
    ///
    /// let _ = Error::unknown_option("foo");
    /// let _ = Error::unknown_option("bar".to_string());
    /// ```
    pub fn unknown_option<S: Into<String>>(option_name: S) -> Error {
        Error::UnknownOption(option_name.into()).into()
    }
}
