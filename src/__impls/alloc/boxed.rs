use crate::reflect::*;
use crate::{Field, Type};

extern crate alloc;
use alloc::boxed::*;

impl<T: ?Sized + Reflected> Reflected for Box<T> {
    fn name() -> String {
        format!("alloc::boxed::Box<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_tuple_struct::<Box<T>>()
    }
}

impl<T: ?Sized + Reflected> ReflectedTupleStruct for Box<T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}
