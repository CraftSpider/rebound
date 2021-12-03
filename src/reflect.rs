//! Reflection related traits

use crate::info::UnionField;
use crate::utils::StaticTypeMap;
use crate::{AssocConst, AssocFn, Error, Field, Type, Value, Variant};

use core::ptr;
use once_cell::sync::OnceCell;
use rebound_proc::impl_find;

/// A trait representing any reflected [`Type`]. Supports operations common to all Types,
/// such as retrieving its qualified name or impl information.
pub trait Reflected {
    /// The static key type used for the backing [`TypeId`] of a Type
    type Key: ?Sized + 'static;

    /// Get the qualified name of this Type
    fn name() -> String;

    /// Get all the associated functions for this Type that rebound is aware of
    fn assoc_fns() -> &'static [AssocFn] {
        static ASSOC_FNS: OnceCell<StaticTypeMap<Vec<AssocFn>>> = OnceCell::new();

        ASSOC_FNS
            .get_or_init(StaticTypeMap::new)
            .call_once::<Self, _>(|| {
                let mut sum = Vec::new();
                impl_find!(assoc_fns);
                sum
            })
    }

    /// Get all the associated constants for this Type that rebound is aware of
    fn assoc_consts() -> &'static [AssocConst] {
        static ASSOC_CONSTS: OnceCell<StaticTypeMap<Vec<AssocConst>>> = OnceCell::new();

        ASSOC_CONSTS
            .get_or_init(StaticTypeMap::new)
            .call_once::<Self, _>(|| {
                let mut sum = Vec::new();
                impl_find!(assoc_consts);
                sum
            })
    }

    /// Internal Function used to initialize this Type, making it accessible by name and ready
    /// for use in reflection.
    ///
    /// # Safety
    ///
    /// Should only ever be called once during a single execution of the program, and should not
    /// be called by consumers of this library.
    unsafe fn init();
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
        unsafe {
            Ok(core::mem::transmute::<Value<'_>, Value<'_>>(Value::from(
                val.borrow_unsafe::<Self>(),
            )))
        }
    }

    default fn mut_val<'a>(val: &'a mut Value<'_>) -> Result<Value<'a>, Error> {
        unsafe {
            Ok(core::mem::transmute::<Value<'_>, Value<'_>>(Value::from(
                val.borrow_unsafe_mut::<Self>(),
            )))
        }
    }
}

impl<T: ?Sized + Reflected> Ref for &T {
    fn ref_val<'a>(val: &'a Value<'_>) -> Result<Value<'a>, Error> {
        unsafe {
            let new_ref = ptr::from_raw_parts::<T>(val.raw_ptr().cast(), *val.raw_meta().cast())
                .as_ref()
                .unwrap();
            Ok(core::mem::transmute::<Value<'_>, Value<'_>>(Value::from(
                new_ref,
            )))
        }
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
