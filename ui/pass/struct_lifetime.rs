
use rebound::{Type, Value, rebound};

#[rebound]
struct ReferenceLifetime<'a>(&'a i32);

#[rebound]
struct PathLifetime<'a>(ReferenceLifetime<'a>);

#[rebound]
struct ArrayLifetime<'a>([&'a i32; 1]);

fn main() {
    let a = 1;

    let rl = ReferenceLifetime(&a);
    let pl = PathLifetime(rl);
    let _al = ArrayLifetime([&a]);

    Type::from::<ReferenceLifetime>();
    Type::from::<PathLifetime>();
    Type::from::<ArrayLifetime>();

    // Check that generated NotOutlives allows borrowing for valid situations
    let val = Value::from(pl);
    let _pl2 = val.borrow::<PathLifetime<'_>>();
}
