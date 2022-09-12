
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
    Type::of::<GenericStruct<i32>>();
    Type::of::<GenericStruct<bool>>();

    Type::of::<ConstGenericStruct<1>>();
    Type::of::<ConstGenericStruct<5>>();
}
