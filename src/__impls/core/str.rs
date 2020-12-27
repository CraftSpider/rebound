use core::str::*;

use rebound_proc::extern_items;

// TODO: Real type of `!` is private / unimplemented

extern_items! {
    pub struct Bytes<'a>(pub(super) core::iter::Copied<core::slice::Iter<'a, u8>>);

    pub struct CharIndices<'a> {
        pub(super) front_offset: usize,
        pub(super) iter: Chars<'a>,
    }

    pub struct Chars<'a> {
        pub(super) iter: core::slice::Iter<'a, u8>,
    }

    pub struct EncodeUtf16<'a> {
        pub(super) chars: Chars<'a>,
        pub(super) extra: u16,
    }

    pub struct EscapeDebug<'a> {
        pub(super) inner: core::iter::Chain<
            core::iter::Flatten<core::option::IntoIter<core::char::EscapeDebug>>,
            core::iter::FlatMap<Chars<'a>, core::char::EscapeDebug, !>,
        >,
    }

    pub struct EscapeDefault<'a> {
        pub(super) inner: core::iter::FlatMap<Chars<'a>, core::char::EscapeDefault, !>,
    }

    pub struct EscapeUnicode<'a> {
        pub(super) inner: core::iter::FlatMap<Chars<'a>, core::char::EscapeUnicode, !>,
    }

    pub struct Lines<'a>(pub(super) core::iter::Map<!, !>);

    pub struct SplitAsciiWhitespace<'a> {
        pub(super) inner:
            core::iter::Map<core::iter::Filter<!, !>, !>,
    }

    pub struct SplitWhitespace<'a> {
        pub(super) inner: core::iter::Filter<!, !>,
    }
}
