use core::iter::*;

use rebound_proc::extern_items;

use crate::__impls::PrivateTy;

extern_items! {
    pub struct Copied<I> {
        it: I,
    }

    pub struct Flatten<I: Iterator<Item: IntoIterator>> {
        inner: PrivateTy,
    }

    pub struct FlatMap<I, U: IntoIterator, F> {
        inner: PrivateTy,
    }

    pub struct Chain<A, B> {
        a: Option<A>,
        b: Option<B>,
    }

    pub struct Map<I, F> {
        iter: I,
        f: F,
    }

    pub struct Filter<I, P> {
        iter: I,
        predicate: P,
    }
}
