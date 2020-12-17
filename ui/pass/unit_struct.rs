
use rebound::{Type, rebound};

#[rebound]
struct Unit;

fn main() {
    Type::from::<Unit>();
}
