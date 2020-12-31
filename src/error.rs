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
    /// A [`Value`](crate::Value) with an incorrect [`Type`] was passed to an operation
    WrongType {
        /// The incorrect type passed
        wrong_ty: Type,
        /// The type that was expected
        right_ty: Type,
    },

    /// Attempted to call an [`AssocFn`](crate::AssocFn) without a `self` param, but the referenced
    /// function expects one
    IsDynamic,

    /// Attempted to call an [`AssocFn`](crate::AssocFn) with a `self` param, but the referenced
    /// function doesn't expect one
    IsStatic,

    /// Attempted to call an [`AssocFn`](crate::AssocFn) with more arguments than expected by the
    /// referenced function.
    TooManyArgs,

    /// Attempted to call an [`AssocFn`](crate::AssocFn) with fewer arguments than expected by the
    /// referenced function
    TooFewArgs,

    /// Attempted to perform an operation on a [`Value`](crate::Value) that requires the Value to be
    /// Owned, but it was Borrowed.
    BorrowedValue,

    /// Attempted to perform an operation on an [`AssocFn`](crate::AssocFn) or
    /// [`Field`](crate::Field), which isn't supported by that item.
    UnsupportedOperation,

    /// Attempted to call as_ref/as_mut on a reference, which can't be reborrowed as the desired
    /// reference type.
    CantReborrow,

    /// Attempted to perform an operation on an Enum with a Value of the wrong Variant
    WrongVariant,
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
