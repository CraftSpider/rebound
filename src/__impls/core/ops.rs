use core::ops::*;

use rebound_proc::extern_items;

extern_items! {
    pub struct Range<Idx> {
        /// The lower bound of the range (inclusive).
        pub start: Idx,
        /// The upper bound of the range (exclusive).
        pub end: Idx,
    }
}
