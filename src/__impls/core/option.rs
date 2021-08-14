use core::option::*;

use rebound_proc::extern_items;

use crate::__impls::PrivateTy;

extern_items! {
    pub enum Option<T> {
        None,
        Some(T),
    }

    pub struct IntoIter<A> {
        inner: PrivateTy,
    }
}
