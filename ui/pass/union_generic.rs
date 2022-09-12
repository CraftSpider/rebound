
use rebound::{Type, rebound};

#[rebound]
union GenericUnion<T> {
    a: i32,
    b: core::mem::ManuallyDrop<T>,
}

#[rebound]
union ConstGenericUnion<const N: usize> {
    a: [u8; N],
    b: u128,
}

fn main() {
    Type::of::<GenericUnion<f32>>();
    Type::of::<GenericUnion<bool>>();
}
