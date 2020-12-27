use core::iter::*;

use rebound_proc::extern_items;

// TODO: Real type of `!` is private

extern_items! {
    pub struct Copied<I> {
        it: I,
    }

    pub struct Flatten<I: Iterator<Item: IntoIterator>> {
        inner: !,
    }

    pub struct FlatMap<I, U: IntoIterator, F> {
        inner: !,
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
