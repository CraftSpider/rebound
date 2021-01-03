
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
    Type::from::<GenericEnum<i32>>();
    Type::from::<GenericEnum<GenericEnum<bool>>>();

    Type::from::<ConstGenericEnum<1>>();
    Type::from::<ConstGenericEnum<7>>();
}
