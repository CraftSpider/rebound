use core::result::*;

use rebound_proc::extern_items;

extern_items! {
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
