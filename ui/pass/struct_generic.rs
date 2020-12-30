
use rebound::{Type, rebound};

#[rebound]
struct GenericStruct<T> {
    inner: T
}

#[rebound]
struct ConstGenericStruct<const N: usize> {
    inner: [i32; N],
}

fn main() {
    Type::from::<GenericStruct<i32>>();
    Type::from::<GenericStruct<bool>>();

    Type::from::<ConstGenericStruct<1>>();
    Type::from::<ConstGenericStruct<5>>();
}
