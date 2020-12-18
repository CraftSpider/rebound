//! A crate aiming to implement 'full' reflection in Rust.

// TODO: Remove specialization / hope it gets completed
#![allow(incomplete_features)]

#![feature(min_const_generics, specialization, decl_macro, once_cell)]

#![cfg_attr(feature = "never-type", feature(never_type))]

mod __impls;

pub mod __helpers;

pub mod error;
pub mod value;
pub mod ty;
pub mod tr;
pub mod info;
pub mod reflect;

pub use error::Error;
pub use value::Value;
pub use ty::Type;
pub use tr::Trait;
pub use info::{AssocFn, AssocConst, Field, FieldKind, VariantInfo};
pub use crate::reflect::Reflected;

pub fn init_base() {
    init_tys!(
        bool,
        char,
        &str,

        u8,
        u16,
        u32,
        u64,
        u128,
        usize,
        i8,
        i16,
        i32,
        i64,
        i128,
        isize,

        f32,
        f64,
    );
}

pub macro init_tys($($ty:ty),+ $(,)?) {
    $( Type::from::<$ty>(); )+
}

extern crate rebound_proc;
#[doc(hidden)]
pub use rebound_proc::rebound;
