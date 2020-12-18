use core::fmt;
use std::error;

// TODO: Improve error display

#[derive(Debug)]
pub enum Error {
    WrongType,

    ExpectedSelf,
    UnexpectedSelf,

    TooManyArgs,
    TooFewArgs,

    InvalidValue
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Reflection Error: {:?}", self)
    }
}

impl error::Error for Error {}
