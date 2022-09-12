
use rebound::{Type, rebound};

#[rebound]
enum Foo {
    UnitVariant,
    TupleVariant(i32),
    StructVariant { bar: bool },
}

fn main() {
    Type::of::<Foo>();
}
