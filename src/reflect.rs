//! Reflection related traits

use crate::info::UnionField;
use crate::utils::StaticTypeMap;
use crate::{AssocConst, AssocFn, Error, Field, Type, Value, Variant};

use rebound_proc::impl_find;

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
    /// The static key type used for the backing [`TypeId`](core::any::TypeId) of a Type
    type Key: ?Sized + Reflected<Key = Self::Key> + 'static;

    /// Get the `Type` associated with this implementor
    fn ty() -> Type;

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
    fn take_ref<'a>(val: &'a Value<'_>) -> Result<Value<'a>, Error> {
        // SAFETY: Value cannot be safely constructed with a lifetime that outlives the contained object.
        //         As such, we know getting a ref to the internal object will always be valid.
        let new_val = unsafe { val.try_borrow_unsafe::<Self>().map(Value::from) }?;
        // SAFETY: The transmute just conveys the above to the rust compiler, converting the lifetime.
        Ok(unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(new_val) })
    }

    #[doc(hidden)]
    fn take_mut<'a>(val: &'a mut Value<'_>) -> Result<Value<'a>, Error> {
        // SAFETY: See comment in take_ref
        let new_val = unsafe { val.try_borrow_unsafe_mut::<Self>().map(Value::from) }?;
        // SAFETY: See comment in take_ref
        Ok(unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(new_val) })
    }
}

/// A trait representing a reflected tuple. Supports operations specific to tuples
pub trait ReflectedTuple: Reflected {
    /// Retrieve the fields of this Tuple
    fn fields() -> &'static [Field];
}

/// A trait representing a reflected slice. Supports operations specific to slices
pub trait ReflectedSlice: Reflected {
    /// Retrieve the element type of this Slice
    fn element() -> Type;
}

/// A trait representing a reflected array. Supports operations specific to arrays
pub trait ReflectedArray: Reflected {
    /// Retrieve the element type of this Array
    fn element() -> Type;
    /// Retrieve the length of this Array
    fn length() -> usize;
}

/// A trait representing a reflected pointer. Supports operations specific to pointers
pub trait ReflectedPointer: Reflected {
    /// Retrieve the element type of this Pointer
    fn element() -> Type;
    /// Retrieve the mutability of this Pointer
    fn mutability() -> bool;
}

/// A trait representing a reflected reference. Supports operations specific to references
pub trait ReflectedReference: Reflected {
    /// Retrieve the element type of this Reference
    fn element() -> Type;
    /// Retrieve the mutability of this Reference
    fn mutability() -> bool;
}

/// A trait representing a reflected function. Supports operations specific to functions
pub trait ReflectedFunction: Reflected {
    /// Retrieve the argument types of this function
    fn args() -> Vec<Type>;
    /// Retrieve the return type of this function
    fn ret() -> Type;
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

// #[linkme::distributed_slice]
// static REFLECTED_IMPLS: [ReflectedImpl] = [..];
//
// // TODO: Figure out how to determine applicability
// pub struct ReflectedImpl {
//     fns: &'static [AssocFn],
//     consts: &'static [AssocConst],
// }
