
use rebound::{Type, rebound};

#[rebound]
enum GenericEnum<T> {
    None,
    Some(T)
}

#[rebound]
enum ConstGenericEnum<const N: usize> {
    Foo([i32; N]),
    Bar,
}

fn main() {
    Type::of::<GenericEnum<i32>>();
    Type::of::<GenericEnum<GenericEnum<bool>>>();

    Type::of::<ConstGenericEnum<1>>();
    Type::of::<ConstGenericEnum<7>>();
}
