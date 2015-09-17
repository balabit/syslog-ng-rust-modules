use handlebars::RenderError;
use std::fmt::{
    Display,
    Formatter,
};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    Render(RenderError),
    FromUtf8(FromUtf8Error)
}

impl From<RenderError> for Error {
    fn from(error: RenderError) -> Error {
        Error::Render(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::FromUtf8(error)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Render(ref error) => error.description(),
            Error::FromUtf8(ref error) => error.description(),
        }
    }
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Render(ref error) => Some(error),
            Error::FromUtf8(ref error) => Some(error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Error::Render(ref error) => error.fmt(fmt),
            Error::FromUtf8(ref error) => error.fmt(fmt),
        }
    }
}
