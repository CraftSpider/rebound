use crate::reflect::ReflectedStruct;
use crate::{Field, Reflected, Type};

use crate::value::NotOutlives;
use core::mem::*;

// TODO: Support custom unsized impls in extern_items!

unsafe impl<T> Reflected for ManuallyDrop<T>
where
    T: ?Sized + Reflected,
{
    const TYPE: Type = Type::new_struct::<ManuallyDrop<T>>();

    type Key = ManuallyDrop<T::Key>;

    fn name() -> String {
        format!("core::mem::ManuallyDrop<{}>", T::name())
    }
}

impl<T> ReflectedStruct for ManuallyDrop<T>
where
    T: ?Sized + Reflected,
{
    const FIELDS: &'static [Field] = &[Field::new_named(None, None, "value", Self::TYPE, T::TYPE)];
}

unsafe impl<'a, 'b, T> NotOutlives<'b> for ManuallyDrop<T>
where
    'b: 'a,
    T: ?Sized + NotOutlives<'a>,
{
}
