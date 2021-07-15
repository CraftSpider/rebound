
use rebound::Value;

struct WithLifetime<'a>(&'a i32);

fn do_stuff(_: &WithLifetime<'_>) {}

fn main() {
    let invalid: WithLifetime<'_>;
    {
        let a = 1;
        let r = WithLifetime(&a);
        let val = Value::from(r);
        invalid = val.cast();
    }

    // Wait, no!
    do_stuff(invalid);
}
