
use rebound::Type;
use rebound_proc::rebound;

#[rebound]
struct TupleStruct(i32, u32);

fn main() {
    Type::from::<TupleStruct>();
}
