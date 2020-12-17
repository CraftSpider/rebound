// #[allow(unused_imports)]
// use crate::prelude::*;

use crate::{TypeInfo, VariantInfo, NamedField};
use crate::reflect::*;

// TODO: Add impls for all these

impl Reflected for core::alloc::Layout {
    fn name() -> String {
        "core::alloc::Layout".into()
    }

    unsafe fn init() {
        TypeInfo::new_struct::<core::alloc::Layout>();
    }
}

impl ReflectedStruct for core::alloc::Layout {
    fn fields() -> Vec<NamedField> {
        vec![]  // TODO
    }
}

impl Reflected for core::alloc::LayoutError {
    fn name() -> String {
        "core::alloc::LayoutErr".into()
    }

    unsafe fn init() {
        TypeInfo::new_struct::<core::alloc::LayoutError>();
    }
}

impl ReflectedStruct for core::alloc::LayoutError {
    fn fields() -> Vec<NamedField> {
        vec![]  // TODO
    }
}

impl Reflected for core::num::ParseIntError {
    fn name() -> String {
        format!("core::num::ParseIntError")
    }

    unsafe fn init() {
        TypeInfo::new_struct::<core::num::ParseIntError>()
    }
}

impl ReflectedStruct for core::num::ParseIntError {
    fn fields() -> Vec<NamedField>{
        vec![]  // TODO
    }
}

impl<T: Reflected, U: Reflected> Reflected for core::result::Result<T, U> {
    fn name() -> String {
        format!("core::result::Result<{}, {}>", T::name(), U::name())
    }

    unsafe fn init() {
        TypeInfo::new_enum::<core::result::Result<T, U>>()
    }
}

impl<T: Reflected, U: Reflected> ReflectedEnum for core::result::Result<T, U> {
    fn variants() -> Vec<VariantInfo> {
        vec![]  // TODO
    }
}

impl<T: Reflected> Reflected for core::marker::PhantomData<T> {
    fn name() -> String {
        format!("core::marker::PhantomData<{}>", T::name())
    }

    unsafe fn init() {
        TypeInfo::new_unit_struct::<core::marker::PhantomData<T>>()
    }
}

impl<T: Reflected> ReflectedUnitStruct for core::marker::PhantomData<T> {}
