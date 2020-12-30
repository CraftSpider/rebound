//! Reflection related traits

use crate::{AssocConst, AssocFn, Field, Type, Variant};

use rebound_proc::impl_find;
use crate::info::UnionField;

pub trait Reflected {
    type Meta: Copy = ();

    fn name() -> String;

    // TODO: These should cache results, if possible
    fn assoc_fns() -> Vec<AssocFn> {
        let mut sum = Vec::new();

        impl_find!(assoc_fns);

        sum
    }

    fn assoc_consts() -> Vec<AssocConst> {
        let mut sum = Vec::new();

        impl_find!(assoc_consts);

        sum
    }

    fn assemble(meta: Self::Meta, ptr: *mut ()) -> *mut Self;

    fn disassemble(&self) -> (Self::Meta, *mut ());

    unsafe fn init();
}

pub trait ReflectedTuple: Reflected {
    fn fields() -> Vec<Field>;
}

pub trait ReflectedSlice: Reflected {
    fn element() -> Type;
}

pub trait ReflectedArray: Reflected {
    fn element() -> Type;
    fn length() -> usize;
}

pub trait ReflectedPointer: Reflected {
    fn element() -> Type;
    fn mutability() -> bool;
}

pub trait ReflectedReference: Reflected {
    fn element() -> Type;
    fn mutability() -> bool;
}

pub trait ReflectedFunction: Reflected {
    fn args() -> Vec<Type>;
    fn ret() -> Type;
}

pub trait ReflectedStruct: Reflected {
    fn fields() -> Vec<Field>;
}

pub trait ReflectedTupleStruct: Reflected {
    fn fields() -> Vec<Field>;
}

pub trait ReflectedUnitStruct: Reflected {}

pub trait ReflectedEnum: Reflected {
    fn variants() -> Vec<Variant>;
}

pub trait ReflectedUnion: Reflected {
    fn fields() -> Vec<UnionField>;
}

pub trait ReflectedImpl<const N: u8>: Reflected {
    fn assoc_fns() -> Vec<AssocFn>;
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
