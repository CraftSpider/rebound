use crate::{Reflected, Type, Field};
use crate::reflect::ReflectedStruct;

use core::mem::*;

// TODO: Support custom unsized impls in extern_items!

impl<T: ?Sized + Reflected> Reflected for ManuallyDrop<T> {
    type Meta = T::Meta;

    fn name() -> String {
        format!("core::mem::ManuallyDrop<{}>", T::name())
    }

    fn assemble(meta: Self::Meta, ptr: *mut ()) -> *mut Self {
        T::assemble(meta, ptr) as *mut ManuallyDrop<T>
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        T::disassemble(self as &T)
    }

    unsafe fn init() {
        Type::new_struct::<ManuallyDrop<T>>()
    }
}

impl<T: ?Sized + Reflected> ReflectedStruct for ManuallyDrop<T> {
    fn fields() -> Vec<Field> {
        unsafe {
            vec![
                Field::new_named(
                    None,
                    None,
                    "value",
                    Type::from::<ManuallyDrop<T>>(),
                    Type::from::<T>(),
                )
            ]
        }
    }
}
