
use rebound::{Type, rebound};

#[rebound]
pub enum LivesFor<'a> {
    None,
    Some(&'a i32)
}

fn main() {
    Type::of::<LivesFor>();
}
