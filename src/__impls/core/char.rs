use crate::reflect::*;
use crate::{Field, Type};

impl Reflected for core::char::EscapeDebug {
    fn name() -> String {
        "core::char::EscapeDebug".into()
    }

    unsafe fn init() {
        Type::new_tuple_struct::<core::char::EscapeDebug>()
    }
}

impl ReflectedTupleStruct for core::char::EscapeDebug {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for core::char::EscapeDefault {
    fn name() -> String {
        "core::char::EscapeDefault".into()
    }

    unsafe fn init() {
        Type::new_struct::<core::char::EscapeDefault>()
    }
}

impl ReflectedStruct for core::char::EscapeDefault {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for core::char::EscapeUnicode {
    fn name() -> String {
        "core::char::EscapeUnicode".into()
    }

    unsafe fn init() {
        Type::new_struct::<core::char::EscapeUnicode>()
    }
}

impl ReflectedStruct for core::char::EscapeUnicode {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for core::char::ToLowercase {
    fn name() -> String {
        "core::char::ToLowercase".into()
    }

    unsafe fn init() {
        Type::new_tuple_struct::<core::char::ToLowercase>()
    }
}

impl ReflectedTupleStruct for core::char::ToLowercase {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for core::char::ToUppercase {
    fn name() -> String {
        "core::char::ToUppercase".into()
    }

    unsafe fn init() {
        Type::new_tuple_struct::<core::char::ToUppercase>()
    }
}

impl ReflectedTupleStruct for core::char::ToUppercase {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}
