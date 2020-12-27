use core::option::*;

use rebound_proc::extern_items;

// TODO: Real type of `!` is private

extern_items! {
    pub enum Option<T> {
        None,
        Some(T),
    }

    pub struct IntoIter<A> {
        inner: !,
    }
}
