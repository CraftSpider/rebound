use core::ptr::*;

use rebound_proc::extern_items;

extern_items! {
    struct Unique<T: ?Sized> {
        pointer: *const T,
        _marker: core::marker::PhantomData<T>,
    }

    struct NonNull<T: ?Sized> {
        pointer: *const T,
    }
}
