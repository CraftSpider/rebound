
use rebound::{Type, rebound};

#[rebound]
enum GenericEnum<T> {
    None,
    Some(T)
}

fn main() {
    Type::from::<GenericEnum<i32>>();
    Type::from::<GenericEnum<GenericEnum<bool>>>();
}
