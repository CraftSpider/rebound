
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
    Type::from::<GenericUnion<f32>>();
    Type::from::<GenericUnion<bool>>();
}
