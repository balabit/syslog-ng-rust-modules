use serde_json;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    SerdeJson(serde_json::error::Error)
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
