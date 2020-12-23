extern crate alloc;
use alloc::string::*;

use rebound_proc::extern_items;

extern_items! {
    pub struct String {
        vec: Vec<u8>,
    }
}
