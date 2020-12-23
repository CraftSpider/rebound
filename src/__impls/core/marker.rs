use core::marker::*;

use rebound_proc::extern_items;

extern_items! {
    struct PhantomData<T: ?Sized>;
}
