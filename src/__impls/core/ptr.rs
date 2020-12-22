use crate::reflect::*;
use crate::{Field, Type};

use core::ptr::*;

impl<T: ?Sized + Reflected> Reflected for Unique<T> {
    fn name() -> String {
        format!("core::ptr::Unique<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<Unique<T>>()
    }
}

impl<T: ?Sized + Reflected> ReflectedStruct for Unique<T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}
