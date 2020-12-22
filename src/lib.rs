//! A crate aiming to implement 'full' reflection in Rust.

// TODO: Remove specialization / hope it gets completed
#![allow(incomplete_features)]
#![feature(min_const_generics, specialization, decl_macro, once_cell)]
#![cfg_attr(feature = "never-type", feature(never_type))]

// Features used just to implement reflection for their stuff
#![feature(ptr_internals)]
mod __impls;

pub mod __helpers;

pub mod error;
pub mod info;
pub mod reflect;
pub mod tr;
pub mod ty;
pub mod value;

pub use crate::reflect::Reflected;
pub use error::Error;
pub use info::{AssocConst, AssocFn, Field, FieldKind, Variant};
pub use tr::Trait;
pub use ty::Type;
pub use value::Value;

/// Pre-initialize some common primitive types
pub fn init_base() {
    init_tys!(
        bool, char, str, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
    );
}

/// Generate code to pre-initialize types, including references and pointers to the type
pub macro init_tys($($ty:ty),+ $(,)?) {
    $(
        Type::from::<$ty>();
        Type::from::<&$ty>();
        Type::from::<&mut $ty>();
        Type::from::<*const $ty>();
        Type::from::<*mut $ty>();
    )+
}

extern crate rebound_proc;
#[doc(hidden)]
pub use rebound_proc::rebound;
