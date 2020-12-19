use crate::reflect::*;
use crate::{Field, Type};

extern crate alloc;
use alloc::vec::*;

impl<T: Reflected> Reflected for Vec<T> {
    fn name() -> String {
        format!("alloc::vec::Vec<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<Vec<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for Vec<T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}
