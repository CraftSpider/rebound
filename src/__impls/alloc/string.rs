use crate::reflect::*;
use crate::{Field, Type};

extern crate alloc;
use alloc::string::*;

impl Reflected for String {
    fn name() -> String {
        "alloc::string::String".into()
    }

    unsafe fn init() {
        Type::new_struct::<String>()
    }
}

impl ReflectedStruct for String {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}
