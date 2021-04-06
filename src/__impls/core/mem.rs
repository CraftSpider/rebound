use crate::reflect::ReflectedStruct;
use crate::{Field, Reflected, Type};

use core::mem::*;

// TODO: Support custom unsized impls in extern_items!

impl<T: ?Sized + Reflected> Reflected for ManuallyDrop<T> {
    type Key = ManuallyDrop<T::Key>;

    fn name() -> String {
        format!("core::mem::ManuallyDrop<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<ManuallyDrop<T>>()
    }
}

impl<T: ?Sized + Reflected> ReflectedStruct for ManuallyDrop<T> {
    fn fields() -> Vec<Field> {
        unsafe {
            vec![Field::new_named(
                None,
                None,
                "value",
                Type::from::<ManuallyDrop<T>>(),
                Type::from::<T>(),
            )]
        }
    }
}
