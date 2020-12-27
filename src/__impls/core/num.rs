use core::num::*;

use rebound_proc::extern_items;

extern_items! {
    pub enum IntErrorKind {
        Empty,
        InvalidDigit,
        PosOverflow,
        NegOverflow,
        Zero,
    }

    pub struct ParseIntError {
        pub(super) kind: IntErrorKind,
    }

    pub struct NonZeroU8(u8);

    pub struct NonZeroU32(u32);

    pub struct NonZeroU64(u64);

    pub struct NonZeroU128(u128);

    pub struct NonZeroUsize(usize);

    pub struct NonZeroI8(i8);

    pub struct NonZeroI16(i16);

    pub struct NonZeroI32(i32);

    pub struct NonZeroI64(i64);

    pub struct NonZeroI128(i128);

    pub struct NonZeroIsize(isize);

    pub struct NonZeroU16(u16);
}
