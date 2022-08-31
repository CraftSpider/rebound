//! Reflection related traits

use crate::info::UnionField;
use crate::utils::StaticTypeMap;
use crate::{AssocConst, AssocFn, Error, Field, Type, Value, Variant};

use rebound_proc::impl_find;
use std::ptr::NonNull;

/// A trait representing any reflected [`Type`]. Supports operations common to all Types,
/// such as retrieving its qualified name or impl information.
///
/// # Safety
///
/// This trait should only be implemented through usage of the `#[rebound]` proc-macro.
///
/// More specifically, unsafe code is allowed to assume that:
/// - The Key type is equivalent to `Self` where all lifetimes are replaced with `'static`
/// - The TYPE constant is the result of calling `Type::new_*::<Self>()`
pub unsafe trait Reflected {
    /// The static key type used for the backing [`TypeId`](core::any::TypeId) of a Type
    type Key: ?Sized + 'static;

    /// The [`Type`] instance associated with this Type
    const TYPE: Type;

    /// Get the qualified name of this Type
    fn name() -> String;

    /// Get all the associated functions for this Type that rebound is aware of
    fn assoc_fns() -> &'static [AssocFn] {
        static ASSOC_FNS: StaticTypeMap<Vec<AssocFn>> = StaticTypeMap::new();

        ASSOC_FNS
            .call_once::<Self, fn() -> _>(|| {
                let mut sum = Vec::new();
                impl_find!(assoc_fns);
                sum
            })
    }

    /// Get all the associated constants for this Type that rebound is aware of
    fn assoc_consts() -> &'static [AssocConst] {
        static ASSOC_CONSTS: StaticTypeMap<Vec<AssocConst>> = StaticTypeMap::new();

        ASSOC_CONSTS
            .call_once::<Self, fn() -> _>(|| {
                let mut sum = Vec::new();
                impl_find!(assoc_consts);
                sum
            })
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
    const ELEMENT: Type;
}

/// A trait representing a reflected array. Supports operations specific to arrays
pub trait ReflectedArray: Reflected {
    /// The length of this array
    const LENGTH: usize;

    /// Retrieve the element type of this Array
    const ELEMENT: Type;
}

/// A trait representing a reflected pointer. Supports operations specific to pointers
pub trait ReflectedPointer: Reflected {
    /// The mutability of this pointer
    const MUTABILITY: bool;

    /// Retrieve the element type of this Pointer
    const ELEMENT: Type;
}

/// A trait representing a reflected reference. Supports operations specific to references
pub trait ReflectedReference: Reflected {
    /// The mutability of this reference
    const MUTABILITY: bool;

    /// The element type of this Reference
    const ELEMENT: Type;
}

/// A trait representing a reflected function. Supports operations specific to functions
pub trait ReflectedFunction: Reflected {
    /// Retrieve the argument types of this function
    const ARGS: &'static [Type];
    /// Retrieve the return type of this function
    const RET: Type;
}

/// A trait representing a reflected struct. Supports operations specific to structs
pub trait ReflectedStruct: Reflected {
    /// Retrieve the fields of this Struct
    fn fields() -> Vec<Field>;
}

/// A trait representing a reflected tuple struct. Supports operations specific to tuple structs
pub trait ReflectedTupleStruct: Reflected {
    /// Retrieve the fields of this Tuple Struct
    fn fields() -> Vec<Field>;
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
    /// Retrieve the union fields of this Union
    fn fields() -> Vec<UnionField>;
}

/// A trait representing a reflected impl for a type.
pub trait ReflectedImpl<const N: u8> {
    /// Associated function defined in this impl
    fn assoc_fns() -> Vec<AssocFn>;
    /// Associated consts defined in this impl
    fn assoc_consts() -> Vec<AssocConst>;
}

impl<T: ?Sized, const N: u8> ReflectedImpl<N> for T {
    default fn assoc_fns() -> Vec<AssocFn> {
        vec![]
    }
    default fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

// Crate-private auto impls
pub(crate) trait Ref: Reflected {
    fn ref_val<'a>(val: &'a Value<'_>) -> Result<Value<'a>, Error>;
    fn mut_val<'a>(val: &'a mut Value<'_>) -> Result<Value<'a>, Error>;
}

// SAFETY: Value cannot be safely constructed with a lifetime that outlives the contained object.
//         As such, we know getting a ref to the internal object will always be valid.
//         The transmute just conveys this to the rust compiler, converting the lifetime.

impl<T: ?Sized + Reflected> Ref for T {
    default fn ref_val<'a>(val: &'a Value<'_>) -> Result<Value<'a>, Error> {
        // SAFETY: The bounds on this function prevent invalid lifetimes
        let val = unsafe { Value::from(val.borrow_unsafe::<Self>()) };
        // SAFETY: See comment above impls
        unsafe { Ok(core::mem::transmute::<Value<'_>, Value<'_>>(val)) }
    }

    default fn mut_val<'a>(val: &'a mut Value<'_>) -> Result<Value<'a>, Error> {
        // SAFETY: The bounds on this function prevent invalid lifetimes
        let val = unsafe { Value::from(val.borrow_unsafe_mut::<Self>()) };
        // SAFETY: See comment above impls
        unsafe { Ok(core::mem::transmute::<Value<'_>, Value<'_>>(val)) }
    }
}

impl<T: ?Sized + Reflected> Ref for &T {
    fn ref_val<'a>(val: &'a Value<'_>) -> Result<Value<'a>, Error> {
        // SAFETY: Value pointer guaranteed to point to an instance of the contained type
        let new_ref = unsafe { NonNull::<&T>::from_raw_parts(val.raw_ptr(), ()).as_ref() };

        let val = Value::from(*new_ref);

        // SAFETY: See comment above impls
        unsafe { Ok(core::mem::transmute::<Value<'_>, Value<'_>>(val)) }
    }

    fn mut_val<'a>(_: &'a mut Value<'_>) -> Result<Value<'a>, Error> {
        Err(Error::CantReborrow)
    }
}

impl<T: ?Sized + Reflected> Ref for &mut T {
    fn ref_val<'a>(_: &'a Value<'_>) -> Result<Value<'a>, Error> {
        Err(Error::CantReborrow)
    }

    fn mut_val<'a>(_: &'a mut Value<'_>) -> Result<Value<'a>, Error> {
        Err(Error::CantReborrow)
    }
}
