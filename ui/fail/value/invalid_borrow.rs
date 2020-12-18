
use rebound::Value;

fn main() {
    let mut val = Value::from(2);

    let borrow = val.borrow::<i32>();
    let _ = val.borrow_mut::<i32>();

    println!("Borrowed over mutable: {}", borrow);
}
