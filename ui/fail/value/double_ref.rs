//! This test should *not* pass, but does, due to limitations
//! in expressing lifetimes. It is here so that the issue being
//! fixed will not go unnoticed.

use rebound::Value;

fn do_stuff(_: &i32) {}

fn main() {
    let invalid: &i32;
    {
        let a = 1;
        let r = &a;
        let val = Value::from(&r);
        invalid = *val.borrow::<&&i32>();
    }

    // Wait, no!
    do_stuff(invalid);
}
