
use rebound::Value;

fn main() {
    let val;
    {
        let lives_shorter = 2;
        val = Value::from_ref(&lives_shorter);
    }

    let _borrow = val.borrow::<i32>();
}
