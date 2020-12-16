
use rebound::TypeInfo;
use rebound_proc::rebound;

#[rebound]
struct NamedStruct {
    a: i32,
    b: u32,
    c: char
}

fn main() {
    TypeInfo::from::<NamedStruct>();
}
