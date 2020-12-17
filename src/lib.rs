//! A crate aiming to implement 'full' reflection in Rust.

// TODO: Find a way to not need specialization
#![allow(incomplete_features)]

#![feature(min_const_generics, specialization, decl_macro)]

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reflect::*;
    use crate::__helpers::*;
    use crate::ty::CommonTypeInfo;

    #[rebound(crate_name = "crate")]
    #[derive(Debug)]
    struct Foo {
        a: i32
    }

    //#[rebound(crate_name = "crate")]
    impl Foo {
        fn new() -> Foo {
            println!("New Foo");

            Foo {
                a: 1
            }
        }

        fn do_thing(&self) {
            println!("Doing Thing");
        }
    }

    impl ReflectedImpl<0> for Foo {
        fn assoc_fns() -> Option<Vec<AssocFn>> {
            unsafe {
                Some(vec![
                    AssocFn::new(
                        __make_static_helper!(Foo::new),
                        "new",
                        Type::from::<Foo>(),
                        None,
                        &[],
                        Type::from::<Foo>()
                    ),
                    AssocFn::new(
                        __make_dyn_helper!(Foo::do_thing, &Foo),
                        "do_thing",
                        Type::from::<Foo>(),
                        Some(Type::from::<&Foo>()),
                        &[],
                        Type::from::<()>()
                    )
                ])
            }
        }

        fn assoc_consts() -> Option<Vec<AssocConst>> {
            Some(vec![])
        }
    }

    #[test]
    fn test_foo() {
        let ty = Type::from::<Foo>();

        let fns = ty.assoc_fns();

        let foo = fns[0].call(None, Vec::new())
            .unwrap();

        println!("{:?}", unsafe { foo.cast::<Foo>() });
    }
}
