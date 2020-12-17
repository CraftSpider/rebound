
use rebound::{Type, rebound};

#[rebound]
struct TupleStruct(i32, u32);

fn main() {
    Type::from::<TupleStruct>();
}
