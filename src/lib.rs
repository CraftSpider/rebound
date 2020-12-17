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

    #[derive(Debug)]
    struct Test<'a> {
        r: &'a i32
    }

    impl Reflected for Test<'_> {
        fn name() -> String {
            concat!(module_path!(), "::", "Test").into()
        }

        unsafe fn init() {
            Type::new_struct::<Test>();
        }
    }

    impl ReflectedStruct for Test<'_> {
        fn fields() -> Vec<Field> {
            unsafe {
                vec![
                    Field::new_named(
                        __helpers::__make_ref_accessor!(Test, r),
                        __helpers::__make_setter!(Test, r),
                        "r",
                        Type::from::<Test>(),
                        Type::from::<&i32>(),
                    )
                ]
            }
        }
    }

    struct Test2<'a> {
        r: std::marker::PhantomData<&'a ()>
    }

    impl Reflected for Test2<'_> {
        fn name() -> String {
            concat!(module_path!(), "::", "Test2").into()
        }

        unsafe fn init() {
            Type::new_struct::<Test2>()
        }
    }

    impl ReflectedStruct for Test2<'_> {
        fn fields() -> Vec<Field> {
            unsafe {
                vec![
                    Field::new_named(
                        __helpers::__make_ref_accessor!(Test, r),
                        __helpers::__make_setter!(Test, r),
                        "r",
                        Type::from::<Test>(),
                        Type::from::<std::marker::PhantomData<&'_ ()>>(),
                    )
                ]
            }
        }
    }

    #[test]
    #[allow(unused)]
    fn test_lifetime_struct() {
        let mut t;
        let v;
        {
            let i = 4;
            t = Test { r: &i };
            v = Value::from(t);

            let tv = v.borrow::<Test>();

            if let Type::Struct(info) = Type::from::<Test>() {
                let fields = info.fields();

                let val = fields[0].get_ref(&v)
                    .unwrap()
                    .borrow::<&i32>();
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
    #[allow(unused)]
    fn test_lifetime_ref() {
        let v;
        {
            let i = 4;
            v = Value::from_ref(&i);

            let t = v.borrow::<i32>();

            // Comment this out and uncomment below to see failure
            drop(v);
        }

        // let t = v.borrow::<i32>();
        // println!("{}", t);
    }
}
