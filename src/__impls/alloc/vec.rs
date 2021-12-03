extern crate alloc;

use alloc::vec::*;

use rebound_proc::extern_items;

use crate::__impls::PrivateTy;

extern_items! {
    pub struct Vec<T, A: alloc::alloc::Allocator = Global> {
        buf: PrivateTy,
        len: usize,
    }
}
