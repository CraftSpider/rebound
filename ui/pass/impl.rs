
#![feature(specialization)]

use rebound::{Type, rebound};

#[rebound]
struct Foo {}

#[rebound(debug_out)]
impl Foo {
    fn new() -> Foo {
        Foo {}
    }
    fn do_stuff(&self) {}
}

fn main() {
    Type::from::<Foo>();
}
