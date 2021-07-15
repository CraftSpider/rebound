use crate::reflect::ReflectedStruct;
use crate::{Field, Reflected, Type};

use core::mem::*;
use crate::value::NotOutlives;

// TODO: Support custom unsized impls in extern_items!

impl<T> Reflected for ManuallyDrop<T>
where
    T: ?Sized + Reflected
{
    type Key = ManuallyDrop<T::Key>;

    fn name() -> String {
        format!("core::mem::ManuallyDrop<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<ManuallyDrop<T>>()
    }
}

impl<T> ReflectedStruct for ManuallyDrop<T>
where
    T: ?Sized + Reflected
{
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

unsafe impl<'a, 'b, T> NotOutlives<'b> for ManuallyDrop<T>
where
    'b: 'a,
    T: ?Sized + NotOutlives<'a>
{}
