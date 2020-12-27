use core::slice::*;

use rebound_proc::extern_items;

extern_items! {
    pub struct Chunks<'a, T: 'a> {
        v: &'a [T],
        chunk_size: usize,
    }

    pub struct ChunksExact<'a, T: 'a> {
        v: &'a [T],
        rem: &'a [T],
        chunk_size: usize,
    }

    pub struct ChunksExactMut<'a, T: 'a> {
        v: &'a mut [T],
        rem: &'a mut [T],
        chunk_size: usize,
    }

    pub struct ChunksMut<'a, T: 'a> {
        v: &'a mut [T],
        chunk_size: usize,
    }

    pub struct Iter<'a, T: 'a> {
        ptr: core::ptr::NonNull<T>,
        end: *const T,
        _marker: core::marker::PhantomData<&'a T>,
    }

    pub struct IterMut<'a, T: 'a> {
        ptr: core::ptr::NonNull<T>,
        end: *mut T,
        _marker: core::marker::PhantomData<&'a mut T>,
    }

    pub struct RChunks<'a, T: 'a> {
        v: &'a [T],
        chunk_size: usize,
    }

    pub struct RChunksExact<'a, T: 'a> {
        v: &'a [T],
        rem: &'a [T],
        chunk_size: usize,
    }

    pub struct RChunksExactMut<'a, T: 'a> {
        v: &'a mut [T],
        rem: &'a mut [T],
        chunk_size: usize,
    }

    pub struct RChunksMut<'a, T: 'a> {
        v: &'a mut [T],
        chunk_size: usize,
    }

    pub struct Windows<'a, T: 'a> {
        v: &'a [T],
        size: core::num::NonZeroUsize,
    }

    // TODO: Support where bounds
    pub struct Split<'a, T: 'a, P: FnMut(&T) -> bool>
    // where
    //     P: FnMut(&T) -> bool,
    {
        v: &'a [T],
        pred: P,
        finished: bool,
    }
}
