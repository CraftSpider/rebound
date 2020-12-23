extern crate alloc;
use alloc::vec::*;

use rebound_proc::extern_items;

extern_items! {
    pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: alloc::alloc::Allocator = Global> {
        buf: alloc::raw_vec::RawVec<T, A>,
        len: usize,
    };
}
