//! Reflection related traits

use crate::{AssocConst, AssocFn, Field, Type, Variant};

use crate::info::UnionField;
use rebound_proc::impl_find;

/// A trait representing any reflected [`Type`]. Supports operations common to all Types,
/// such as retrieving its qualified name or impl information.
pub trait Reflected {
    /// The type of any meta-information required to get back a pointer to this type,
    /// if it is unsized
    type Meta: Copy = ();

    /// Get the qualified name of this Type
    fn name() -> String;

    /// Get all the associated functions for this Type that rebound is aware of
    // TODO: These should cache results, if possible
    fn assoc_fns() -> Vec<AssocFn> {
        let mut sum = Vec::new();

        impl_find!(assoc_fns);

        sum
    }

    /// Get all the associated constants for this Type that rebound is aware of
    fn assoc_consts() -> Vec<AssocConst> {
        let mut sum = Vec::new();

        impl_find!(assoc_consts);

        sum
    }

    /// Internal Function used to create a pointer to this type from Metadata and a type-erased
    /// pointer.
    fn assemble(meta: Self::Meta, ptr: *mut ()) -> *mut Self;

    /// Internal Function used to get the metadata and a type-erased pointer for an instance of
    /// this Type.
    fn disassemble(&self) -> (Self::Meta, *mut ());

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
    fn fields() -> Vec<Field>;
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
