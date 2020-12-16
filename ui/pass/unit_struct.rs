
use rebound::TypeInfo;
use rebound_proc::rebound;

#[rebound]
struct Unit;

fn main() {
    TypeInfo::from::<Unit>();
}
