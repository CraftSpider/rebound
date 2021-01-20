//! Error utilities for rebound

use crate::{Type, Variant};

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

    /// Attempted to call an [`AssocFn`](crate::AssocFn) with different number of arguments than
    /// expected by the referenced function.
    WrongArgsNum {
        /// The number of args passed
        wrong_num: usize,
        /// The number of args expected
        right_num: usize,
    },

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
    WrongVariant {
        /// The enum variant provided
        wrong_var: Variant,
        /// The enum variant expected
        right_var: Variant,
    },
}

impl Error {
    pub(crate) fn wrong_type(wrong: Type, right: Type) -> Error {
        Error::WrongType {
            wrong_ty: wrong,
            right_ty: right,
        }
    }

    pub(crate) fn wrong_args_num(wrong: usize, right: usize) -> Error {
        Error::WrongArgsNum {
            wrong_num: wrong,
            right_num: right,
        }
    }

    pub(crate) fn wrong_variant(wrong: &Variant, right: &Variant) -> Error {
        Error::WrongVariant {
            wrong_var: wrong.clone(),
            right_var: right.clone(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::WrongType { wrong_ty, right_ty } => write!(
                f,
                "Reflection Error: Expected type \"{}\", got type \"{}\"",
                right_ty.name(),
                wrong_ty.name()
            ),
            Error::IsDynamic => write!(
                f,
                "Reflection Error: Attempted to call a dynamic function without a receiver to bind"
            ),
            Error::IsStatic => write!(
                f,
                "Reflection Error: Attempted to call a static function with a receiver instance"
            ),
            Error::WrongArgsNum { wrong_num, right_num } => write!(
                f,
                "Reflection Error: Expected {} arguments, got {}",
                wrong_num,
                right_num,
            ),
            Error::BorrowedValue => write!(
                f,
                "Reflection Error: This operation requires an Owned Value, but the provided Value was Borrowed"
            ),
            Error::UnsupportedOperation => write!(
                f,
                "Reflection Error: This operation is not supported by this object. Likely, the #[rebound] macro for this object had a `no_*` attribute"
            ),
            Error::CantReborrow => write!(
                f,
                "Reflection Error: Attempted to get a mutable reference to an existing reference, or any reference to an existing mutable reference"
            ),
            Error::WrongVariant { wrong_var, right_var } => write!(
                f,
                "Reflection Error: Expected enum variant \"{}\", got variant \"{}\"",
                wrong_var.name(),
                right_var.name()
            ),
        }
    }
}

impl error::Error for Error {}
