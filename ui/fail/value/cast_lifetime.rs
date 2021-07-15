use rebound::Value;

fn do_stuff(_: &i32) {}

fn main() {
    let invalid: &i32;
    {
        let a = 1;
        let v = Value::from(&a);
        invalid = v.cast();
    }

    // Wait, no!
    do_stuff(invalid);
}
