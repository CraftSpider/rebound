
extern crate alloc;
use alloc::raw_vec::*;

use rebound_proc::extern_items;

extern_items! {
    pub struct RawVec<T, A: alloc::alloc::Allocator = Global> {
        ptr: core::ptr::Unique<T>,
        cap: usize,
        alloc: A,
    }
}
