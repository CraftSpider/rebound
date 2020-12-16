
use rebound::TypeInfo;
use rebound_proc::rebound;

#[rebound]
struct TupleStruct(i32, u32);

fn main() {
    TypeInfo::from::<TupleStruct>();
}
