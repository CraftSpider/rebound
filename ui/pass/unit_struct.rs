
use rebound::Type;
use rebound_proc::rebound;

#[rebound]
struct Unit;

fn main() {
    Type::from::<Unit>();
}
