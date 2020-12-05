use core::fmt;

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

#[cfg(feature = "std")]
use std::error;

#[cfg(feature = "std")]
impl error::Error for Error {

}
