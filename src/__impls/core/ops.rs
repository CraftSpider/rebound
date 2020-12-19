use crate::reflect::*;
use crate::{Type, Field};

use core::ops::*;

impl<T: Reflected> Reflected for Range<T> {
    fn name() -> String {
        format!("core::ops::Range<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<Range<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for Range<T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}
