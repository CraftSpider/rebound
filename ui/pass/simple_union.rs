
use rebound::{Type, rebound};

#[rebound]
union Foo {
    a: i32,
    b: f32,
}

fn main() {
    Type::from::<Foo>();
}
