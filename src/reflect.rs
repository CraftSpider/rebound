// #[allow(unused_imports)]
// use crate::prelude::*;

use crate::{Type, AssocFn, AssocConst, TupleField, VariantInfo, NamedField};

use rebound_proc::impl_find;

pub trait Reflected {
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

    unsafe fn init();
}

pub trait ReflectedTuple: Reflected {
    fn fields() -> Vec<TupleField>;
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

pub trait ReflectedStruct: Reflected {
    fn fields() -> Vec<NamedField>;
}

pub trait ReflectedEnum: Reflected {
    fn variants() -> Vec<VariantInfo>;
}

pub trait ReflectedTupleStruct: Reflected {
    fn fields() -> Vec<TupleField>;
}

pub trait ReflectedUnitStruct: Reflected {}

pub trait ReflectedImpl<const N: u8>: Reflected {
    fn assoc_fns() -> Option<Vec<AssocFn>>;
    fn assoc_consts() -> Option<Vec<AssocConst>>;
}

impl<T: ?Sized + Reflected, const N: u8> ReflectedImpl<N> for T {
    default fn assoc_fns() -> Option<Vec<AssocFn>> {
        None
    }
    default fn assoc_consts() -> Option<Vec<AssocConst>> {
        None
    }
}
