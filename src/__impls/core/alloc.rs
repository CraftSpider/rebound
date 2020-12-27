use core::alloc::*;

use rebound_proc::extern_items;

extern_items! {
    pub struct Layout {
        size_: usize,
        align_: core::num::NonZeroUsize,
    }

    pub struct LayoutError {
        private: (),
    }
}
