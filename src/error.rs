//! Error utilities for rebound

use crate::Type;

use crate::ty::CommonTypeInfo;
use core::fmt;
use std::error;

/// An error during a reflection operation. Note that during, for example, a call that *returns*
/// an Error, this will not represent the call returning an error. It would instead represent the
/// call failing to occur, or panicking and that panic being caught.
#[derive(Debug)]
pub enum Error {
    WrongType { wrong_ty: Type, right_ty: Type },

    ExpectedSelf,
    UnexpectedSelf,

    TooManyArgs,
    TooFewArgs,

    InvalidValue,
    UnsupportedOperation,
}

impl Error {
    pub(crate) fn wrong_type(wrong: Type, right: Type) -> Error {
        Error::WrongType {
            wrong_ty: wrong,
            right_ty: right,
        }
    }
}

// TODO: Improve error display
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::WrongType { wrong_ty, right_ty } => write!(
                f,
                "Reflection Error: Expected type \"{}\", got type \"{}\"",
                right_ty.name(),
                wrong_ty.name()
            ),
            _ => write!(f, "Reflected Error: {:?}", self),
        }
    }
}

impl error::Error for Error {}
