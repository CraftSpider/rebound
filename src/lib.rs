//! A crate aiming to implement 'full' reflection in Rust.

#![allow(incomplete_features, clippy::nonstandard_macro_braces)]
#![warn(
    missing_docs,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    missing_abi,
    noop_method_call,
    pointer_structural_match,
    semicolon_in_expressions_from_macros,
    unused_import_braces,
    unused_lifetimes,
    clippy::cargo,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::ptr_as_ptr,
    clippy::cloned_instead_of_copied,
    clippy::unreadable_literal
)]
#![feature(specialization, ptr_metadata)]
// Features used just to implement reflection for their stuff
#![cfg_attr(feature = "never-type", feature(never_type))]
#![feature(associated_type_bounds, allocator_api, ptr_internals)]

#[doc(hidden)]
pub use rebound_proc::rebound;

pub use error::Error;
pub use info::{AssocConst, AssocFn, Field, FieldKind, UnionField, Variant};
pub use ty::Type;
pub use value::Value;

pub use crate::reflect::Reflected;

mod __impls;

pub mod error;
pub mod info;
pub mod reflect;
pub mod tr;
pub mod ty;
pub mod value;

pub mod utils;

/// Pre-initialize some common primitive types
pub fn init_base() {
    init_tys!(
        bool, char, str, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
    );
}

/// Generate code to pre-initialize types, including references and pointers to the type
#[macro_export]
macro_rules! init_tys {
    ($($ty:ty),+ $(,)?) => {
        $(
            Type::from::<$ty>();
            Type::from::<&$ty>();
            Type::from::<&mut $ty>();
            Type::from::<*const $ty>();
            Type::from::<*mut $ty>();
        )+
    }
}

extern crate rebound_proc;
