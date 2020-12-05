//! A crate aiming to implement 'full' reflection in Rust.

#![feature(min_const_generics, specialization, decl_macro)]

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "never-type", feature(never_type))]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;
#[cfg(not(feature = "std"))]
mod prelude;
#[cfg(feature = "std")]
mod prelude {}

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
pub use ty::{Type, TypeInfo};
pub use tr::{Trait, TraitInfo};
pub use info::{AssocFn, AssocConst, TupleField, VariantInfo, NamedField};
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
    $( TypeInfo::from::<$ty>(); )+
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reflect::*;
    use crate::__helpers::*;
    use crate::ty::CommonTypeInfo;

    #[derive(Debug)]
    //#[rebound]
    struct Foo {
        a: i32
    }

    impl Reflected for Foo {
        fn name() -> String {
            concat!(module_path!(), "::", "Foo").into()
        }

        unsafe fn init() {
            TypeInfo::new_struct::<Foo>()
        }
    }

    impl ReflectedStruct for Foo {
        fn fields() -> Vec<NamedField> {
            unsafe {
                vec![
                    NamedField::new(
                        __make_ref_accessor!(Foo, a),
                        __make_setter!(Foo, a),
                        "a",
                        TypeInfo::from::<Foo>(),
                        TypeInfo::from::<i32>(),
                    )
                ]
            }
        }
    }

    //#[rebound]
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
                        TypeInfo::from::<Foo>(),
                        None,
                        &[],
                        TypeInfo::from::<Foo>()
                    ),
                    AssocFn::new(
                        __make_dyn_helper!(Foo::do_thing, &Foo),
                        "do_thing",
                        TypeInfo::from::<Foo>(),
                        Some(TypeInfo::from::<&Foo>()),
                        &[],
                        TypeInfo::from::<()>()
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
        let ty = TypeInfo::from::<Foo>();

        let fns = ty.assoc_fns();

        println!("{:?}", fns);

        let foo = fns[0].call(None, Vec::new())
            .unwrap();

        println!("{:?}", unsafe { foo.cast::<Foo>() });
    }

    #[derive(Debug)]
    struct Test<'a> {
        r: &'a i32
    }

    impl Reflected for Test<'_> {
        fn name() -> String {
            concat!(module_path!(), "::", "Test").into()
        }

        unsafe fn init() {
            TypeInfo::new_struct::<Test>();
        }
    }

    impl ReflectedStruct for Test<'_> {
        fn fields() -> Vec<NamedField> {
            unsafe {
                vec![
                    NamedField::new(
                        __helpers::__make_ref_accessor!(Test, r),
                        __helpers::__make_setter!(Test, r),
                        "r",
                        TypeInfo::from::<Test>(),
                        TypeInfo::from::<&i32>(),
                    )
                ]
            }
        }
    }

    #[allow(unused_assignments)]
    #[test]
    fn test_lifetime_struct() {
        let mut t;
        let v;
        {
            let i = 4;
            t = Test { r: &i };
            v = Value::from(t);

            let tv = v.borrow::<Test>();
            println!("{}", tv.r);

            if let TypeInfo::Struct(info) = TypeInfo::from::<Test>() {
                let fields = info.fields();
                println!("{:?}", fields);

                let val = fields[0].get_ref(&v)
                    .unwrap()
                    .borrow::<&i32>();

                println!("{}", val);
            }

            // Comment this out and uncomment below to see failure
            // drop(v);
            // FIXME: This should be impossible, Test isn't `'static`, shouldn't allow casting to it
            t = unsafe { v.cast::<Test>() };
        }

        // let t = v.borrow::<Test>();
        // println!("{}", t.r);
    }

    #[test]
    fn test_lifetime_ref() {
        let v;
        {
            let i = 4;
            v = Value::from_ref(&i);

            let t = v.borrow::<i32>();
            println!("{}", t);

            // Comment this out and uncomment below to see failure
            drop(v);
        }

        // let t = v.borrow::<i32>();
        // println!("{}", t);
    }
}
