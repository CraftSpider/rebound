use crate::reflect::*;
use crate::Type;

use core::marker::*;

impl<T: Reflected> Reflected for PhantomData<T> {
    fn name() -> String {
        format!("core::marker::PhantomData<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_unit_struct::<PhantomData<T>>()
    }
}

impl<T: Reflected> ReflectedUnitStruct for PhantomData<T> {}
