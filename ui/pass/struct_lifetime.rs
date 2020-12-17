
use rebound::{Type, rebound};

#[rebound]
struct ReferenceLifetime<'a>(&'a i32);

#[rebound]
struct PathLifetime<'a>(ReferenceLifetime<'a>);

#[rebound]
struct ArrayLifetime<'a>([&'a i32; 1]);

fn main() {
    let a = 1;

    let rl = ReferenceLifetime(&a);
    let _pl = PathLifetime(rl);
    let _al = ArrayLifetime([&a]);

    Type::from::<ReferenceLifetime>();
    Type::from::<PathLifetime>();
    Type::from::<ArrayLifetime>();
}
