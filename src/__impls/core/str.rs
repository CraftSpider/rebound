use crate::reflect::*;
use crate::{Field, Type};

use core::str::*;

impl Reflected for Bytes<'_> {
    fn name() -> String {
        "core::str::Bytes".into()
    }

    unsafe fn init() {
        Type::new_tuple_struct::<Bytes>()
    }
}

impl ReflectedTupleStruct for Bytes<'_> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for CharIndices<'_> {
    fn name() -> String {
        "core::str::CharIndices".into()
    }

    unsafe fn init() {
        Type::new_struct::<CharIndices>()
    }
}

impl ReflectedStruct for CharIndices<'_> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for Chars<'_> {
    fn name() -> String {
        "core::str::Chars".into()
    }

    unsafe fn init() {
        Type::new_struct::<Chars>()
    }
}

impl ReflectedStruct for Chars<'_> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for EncodeUtf16<'_> {
    fn name() -> String {
        "core::str::EncodeUtf16".into()
    }

    unsafe fn init() {
        Type::new_struct::<EncodeUtf16>()
    }
}

impl ReflectedStruct for EncodeUtf16<'_> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for EscapeDebug<'_> {
    fn name() -> String {
        "core::str::EscapeDebug".into()
    }

    unsafe fn init() {
        Type::new_struct::<EscapeDebug>()
    }
}

impl ReflectedStruct for EscapeDebug<'_> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for EscapeDefault<'_> {
    fn name() -> String {
        "core::str::EscapeDefault".into()
    }

    unsafe fn init() {
        Type::new_struct::<EscapeDefault>()
    }
}

impl ReflectedStruct for EscapeDefault<'_> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for EscapeUnicode<'_> {
    fn name() -> String {
        "core::str::EscapeUnicode".into()
    }

    unsafe fn init() {
        Type::new_struct::<EscapeUnicode>()
    }
}

impl ReflectedStruct for EscapeUnicode<'_> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for Lines<'_> {
    fn name() -> String {
        "core::str::Lines".into()
    }

    unsafe fn init() {
        Type::new_tuple_struct::<Lines>()
    }
}

impl ReflectedTupleStruct for Lines<'_> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for SplitAsciiWhitespace<'_> {
    fn name() -> String {
        "core::str::SplitAsciiWhitespace".into()
    }

    unsafe fn init() {
        Type::new_struct::<SplitAsciiWhitespace>()
    }
}

impl ReflectedStruct for SplitAsciiWhitespace<'_> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl Reflected for SplitWhitespace<'_> {
    fn name() -> String {
        "core::str::SplitWhitespace".into()
    }

    unsafe fn init() {
        Type::new_struct::<SplitWhitespace>()
    }
}

impl ReflectedStruct for SplitWhitespace<'_> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}
