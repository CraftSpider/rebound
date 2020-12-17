
use rebound_proc::rebound;
use rebound::Value;

#[rebound]
struct Lifetime<'a>(&'a i32);

fn main() {
    let val;
    {
        let lives_shorter = 2;
        let life = Lifetime(&lives_shorter);
        val = Value::from(life);
    }

    let _borrow = val.borrow::<i32>();
}
