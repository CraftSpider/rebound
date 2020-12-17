
use rebound::{Type, rebound};

#[rebound]
struct NamedStruct {
    a: i32,
    b: u32,
    c: char
}

fn main() {
    Type::from::<NamedStruct>();
}
