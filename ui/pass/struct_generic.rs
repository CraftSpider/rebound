
use rebound::{Type, rebound};

#[rebound]
struct GenericStruct<T> {
    inner: T
}

fn main() {
    Type::from::<GenericStruct<i32>>();
    Type::from::<GenericStruct<bool>>();
}
