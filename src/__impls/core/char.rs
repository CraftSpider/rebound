use core::char::*;

use rebound_proc::extern_items;

// TODO: Real kinds of `!` are private

extern_items! {
    pub struct EscapeDebug(EscapeDefault);

    pub struct EscapeDefault {
        state: !,
    }

    pub struct EscapeUnicode {
        c: char,
        state: !,
        hex_digit_idx: usize,
    }

    pub struct ToLowercase(!);

    pub struct ToUppercase(!);
}
