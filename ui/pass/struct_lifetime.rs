
use rebound::TypeInfo;
use rebound_proc::rebound;

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
    let al = ArrayLifetime([&a]);

    TypeInfo::from::<ReferenceLifetime>();
    TypeInfo::from::<PathLifetime>();
    TypeInfo::from::<ArrayLifetime>();
}
