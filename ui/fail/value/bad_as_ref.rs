
use rebound::Value;

fn main() {
    let mut v = Value::from(1);
    let v2 = v.as_ref();

    v.borrow_mut::<i32>();

    println!("{:p}", v2);
}
