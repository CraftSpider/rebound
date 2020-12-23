extern crate alloc;
use alloc::boxed::*;

use rebound_proc::extern_items;

extern_items! {
    pub struct Box<T: ?Sized, #[unstable(feature = "allocator_api", issue = "32838")] A: alloc::alloc::Allocator = Global>(core::ptr::Unique<T>, A);
}
