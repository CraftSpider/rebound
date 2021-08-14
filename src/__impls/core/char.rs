use core::char::*;

use rebound_proc::extern_items;

use crate::__impls::PrivateTy;

extern_items! {
    pub struct EscapeDebug(EscapeDefault);

    pub struct EscapeDefault {
        state: PrivateTy,
    }

    pub struct EscapeUnicode {
        c: char,
        state: PrivateTy,
        hex_digit_idx: usize,
    }

    pub struct ToLowercase(PrivateTy);

    pub struct ToUppercase(PrivateTy);
}
