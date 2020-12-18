
#![allow(incomplete_features)]
#![feature(specialization)]

use rebound::{Type, rebound};

#[rebound]
struct Foo {}

#[rebound]
impl Foo {
    const FOO: Foo = Foo {};

    fn new() -> Foo {
        Foo {}
    }
    fn do_stuff(&self) {}
}

fn main() {
    Type::from::<Foo>();
}
