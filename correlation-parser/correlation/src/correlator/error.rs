// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use serde_json;
use serde_yaml;
use std::io;
use std::fmt::{Display, Error as FmtError, Formatter};

use CompileError;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    SerdeJson(serde_json::error::Error),
    SerdeYaml(serde_yaml::error::Error),
    TemplateCompile(CompileError),
    UnsupportedFileExtension(String),
    FileExtensionNotFound,
    NotUtf8FileName
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    }
}

impl From<serde_yaml::error::Error> for Error {
    fn from(error: serde_yaml::error::Error) -> Error {
        Error::SerdeYaml(error)
    }
}

impl From<CompileError> for Error {
    fn from(error: CompileError) -> Error {
        Error::TemplateCompile(error)
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            Error::Io(ref error) => error.fmt(formatter),
            Error::SerdeJson(ref error) => error.fmt(formatter),
            Error::SerdeYaml(ref error) => error.fmt(formatter),
            Error::TemplateCompile(ref error) => error.fmt(formatter),
            Error::UnsupportedFileExtension(ref ext) => formatter.write_fmt(format_args!("File extension '{}' is not supported", ext)),
            Error::FileExtensionNotFound => formatter.write_str("The configuration file does not have an extension"),
            Error::NotUtf8FileName => formatter.write_str("File name is not a valid UTF-8 character sequence"),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref error) => error.description(),
            Error::SerdeJson(ref error) => error.description(),
            Error::SerdeYaml(ref error) => error.description(),
            Error::TemplateCompile(ref error) => error.description(),
            Error::UnsupportedFileExtension(_) => "The correlation library does not support this file format",
            Error::FileExtensionNotFound => "The configuration file does not have file extension",
            Error::NotUtf8FileName => "File name is not a valid UTF-8 character sequence",
        }
    }
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Io(ref error) => error.cause(),
            Error::SerdeJson(ref error) => error.cause(),
            Error::SerdeYaml(ref error) => error.cause(),
            Error::TemplateCompile(ref error) => error.cause(),
            Error::UnsupportedFileExtension(_) |
                Error::FileExtensionNotFound |
                Error::NotUtf8FileName => None,
        }
    }
}
