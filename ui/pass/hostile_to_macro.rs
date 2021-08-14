
#![feature(min_specialization)]

// No implicit std or core imports
#![no_implicit_prelude]

extern crate rebound;

use rebound::rebound;

// Possible confusion of import names
mod core {}
mod std {}

#[rebound]
struct Foo {
    a: i32
}

#[rebound]
impl Foo {
    const BAZ: i32 = 1;

    fn new() -> Foo { ::core::todo!() }
    fn bazzify(&self, _a: i32) { ::core::todo!() }
}

#[rebound]
enum Bar {
    A(i32),
    B { item: bool }
}

fn main() {}
