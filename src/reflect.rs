//! Reflection related traits

use crate::info::UnionField;
use crate::utils::StaticTypeMap;
use crate::{AssocConst, AssocFn, Error, Field, Type, Value, Variant};

use crate::value::NotOutlives;
use rebound_proc::impl_find;

pub(crate) mod sealed {
    use super::*;
    use std::ops::{Deref, DerefMut};

    /// Internal hack to avoid mut refs in `Reflected`
    pub struct RefHack<'a, 'b>(pub(crate) &'a mut Value<'b>);

    impl<'a, 'b> From<&'a mut Value<'b>> for RefHack<'a, 'b> {
        fn from(value: &'a mut Value<'b>) -> Self {
            Self(value)
        }
    }

    impl<'b> Deref for RefHack<'_, 'b> {
        type Target = Value<'b>;

        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    impl<'b> DerefMut for RefHack<'_, 'b> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.0
        }
    }
}

pub(crate) use sealed::RefHack;

/// A trait representing any reflected [`Type`]. Supports operations common to all Types,
/// such as retrieving its qualified name or impl information.
///
/// # Safety
///
/// This trait should only be implemented through usage of the `#[rebound]` proc-macro.
///
/// More specifically, unsafe code is allowed to assume that:
/// - The Key type is equivalent to `Self` where all lifetimes are replaced with `'static`
/// - This trait is **exempt from semver** in terms of internal methods. Methods may be added,
///   removed, or have their signature changed at the whim of the implementation.
pub unsafe trait Reflected {
    /// The `Type` associated with this implementor
    const TYPE: Type;

    /// The static key type used for the backing [`TypeId`](core::any::TypeId) of a Type
    type Key: ?Sized + Reflected<Key = Self::Key> + NotOutlives<'static> + 'static;

    /// Get the qualified name of this Type
    fn name() -> String;

    /// Get all the associated functions for this Type that rebound is aware of
    fn assoc_fns() -> &'static [AssocFn] {
        static ASSOC_FNS: StaticTypeMap<Vec<AssocFn>> = StaticTypeMap::new();

        ASSOC_FNS.call_once::<Self, fn() -> _>(|| {
            let mut sum = Vec::new();
            impl_find!(assoc_fns);
            sum
        })
    }

    /// Get all the associated constants for this Type that rebound is aware of
    fn assoc_consts() -> &'static [AssocConst] {
        static ASSOC_CONSTS: StaticTypeMap<Vec<AssocConst>> = StaticTypeMap::new();

        ASSOC_CONSTS.call_once::<Self, fn() -> _>(|| {
            let mut sum = Vec::new();
            impl_find!(assoc_consts);
            sum
        })
    }

    #[doc(hidden)]
    fn take_ref<'a, 'b>(val: &'a Value<'b>) -> Result<Value<'a>, Error>
    where
        Self: 'a + NotOutlives<'b>,
    {
        val.try_borrow::<Self>().map(Value::from)
    }

    #[doc(hidden)]
    fn take_mut<'a, 'b>(val: RefHack<'a, 'b>) -> Result<Value<'a>, Error>
    where
        Self: 'a + NotOutlives<'b>,
    {
        val.0.try_borrow_mut::<Self>().map(Value::from)
    }
}

/// A trait representing a reflected tuple. Supports operations specific to tuples
pub trait ReflectedTuple: Reflected {
    /// The fields of this Tuple
    const FIELDS: &'static [Field];
}

/// A trait representing a reflected slice. Supports operations specific to slices
pub trait ReflectedSlice: Reflected {
    /// Retrieve the element type of this Slice
    fn element() -> Type;
}

/// A trait representing a reflected array. Supports operations specific to arrays
pub trait ReflectedArray: Reflected {
    /// The length of this Array
    const LENGTH: usize;

    /// Retrieve the element type of this Array
    fn element() -> Type;
}

/// A trait representing a reflected pointer. Supports operations specific to pointers
pub trait ReflectedPointer: Reflected {
    /// The mutability of this Pointer
    const MUTABILITY: bool;

    /// Retrieve the element type of this Pointer
    fn element() -> Type;
}

/// A trait representing a reflected reference. Supports operations specific to references
pub trait ReflectedReference: Reflected {
    /// The mutability of this Reference
    const MUTABILITY: bool;

    /// Retrieve the element type of this Reference
    fn element() -> Type;
}

/// A trait representing a reflected function. Supports operations specific to functions
pub trait ReflectedFunction: Reflected {
    /// Retrieve the argument types of this function
    fn args() -> &'static [Type];
    /// Retrieve the return type of this function
    fn ret() -> Type;
}

/// A trait representing a reflected struct. Supports operations specific to structs
pub trait ReflectedStruct: Reflected {
    /// The fields of this Struct
    const FIELDS: &'static [Field];
}

/// A trait representing a reflected tuple struct. Supports operations specific to tuple structs
pub trait ReflectedTupleStruct: Reflected {
    /// The fields of this Tuple Struct
    const FIELDS: &'static [Field];
}

/// A trait representing a reflected unit struct.
pub trait ReflectedUnitStruct: Reflected {}

/// A trait representing a reflected enum. Supports operations specific to enums
pub trait ReflectedEnum: Reflected {
    /// Retrieve the variants of this Enum
    fn variants() -> Vec<Variant>;
}

/// A trait representing a reflected union. Supports operations specific to unions
pub trait ReflectedUnion: Reflected {
    /// The fields of this Union
    const FIELDS: &'static [UnionField];
}

/// A trait representing a reflected impl for a type.
pub trait ReflectedImpl<const N: u8>: Reflected {
    /// Associated function defined in this impl
    fn assoc_fns() -> Vec<AssocFn>;
    /// Associated consts defined in this impl
    fn assoc_consts() -> Vec<AssocConst>;
}

impl<T: ?Sized + Reflected, const N: u8> ReflectedImpl<N> for T {
    default fn assoc_fns() -> Vec<AssocFn> {
        vec![]
    }
    default fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

// TODO: Figure out how to determine applicability
// pub struct ReflectedImpl {
//     fns: &'static [AssocFn],
//     consts: &'static [AssocConst],
// }
