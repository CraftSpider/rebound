use crate::reflect::ReflectedStruct;
use crate::{Field, Reflected, Type};

use crate::value::NotOutlives;
use core::mem::*;

// TODO: Support custom unsized impls in extern_items!

impl<T> Reflected for ManuallyDrop<T>
where
    T: ?Sized + Reflected,
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
    T: ?Sized + Reflected,
{
    fn fields() -> Vec<Field> {
        // SAFETY: In `fields` implementation and we're the trusted implementation
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
    T: ?Sized + NotOutlives<'a>,
{
}
