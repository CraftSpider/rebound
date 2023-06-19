Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "crate_name", span: #0 bytes(2201..2211) }, arguments: PathArguments::None }] }
mod base {
    use crate::reflect::{
        RefHack, Reflected, ReflectedArray, ReflectedFunction, ReflectedImpl,
        ReflectedPointer, ReflectedReference, ReflectedSlice, ReflectedTuple,
    };
    use crate::value::NotOutlives;
    use crate::{AssocConst, AssocFn, Error, Field, Type, Value};
    use rebound_proc::{extern_assoc_consts, extern_assoc_fns};
    unsafe impl Reflected for u8 {
        const TYPE: Type = Type::new_prim::<u8>();
        type Key = u8;
        fn name() -> String {
            "u8".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for u8 {}
    unsafe impl Reflected for u16 {
        const TYPE: Type = Type::new_prim::<u16>();
        type Key = u16;
        fn name() -> String {
            "u16".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for u16 {}
    unsafe impl Reflected for u32 {
        const TYPE: Type = Type::new_prim::<u32>();
        type Key = u32;
        fn name() -> String {
            "u32".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for u32 {}
    unsafe impl Reflected for u64 {
        const TYPE: Type = Type::new_prim::<u64>();
        type Key = u64;
        fn name() -> String {
            "u64".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for u64 {}
    unsafe impl Reflected for u128 {
        const TYPE: Type = Type::new_prim::<u128>();
        type Key = u128;
        fn name() -> String {
            "u128".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for u128 {}
    unsafe impl Reflected for usize {
        const TYPE: Type = Type::new_prim::<usize>();
        type Key = usize;
        fn name() -> String {
            "usize".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for usize {}
    unsafe impl Reflected for i8 {
        const TYPE: Type = Type::new_prim::<i8>();
        type Key = i8;
        fn name() -> String {
            "i8".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for i8 {}
    unsafe impl Reflected for i16 {
        const TYPE: Type = Type::new_prim::<i16>();
        type Key = i16;
        fn name() -> String {
            "i16".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for i16 {}
    unsafe impl Reflected for i32 {
        const TYPE: Type = Type::new_prim::<i32>();
        type Key = i32;
        fn name() -> String {
            "i32".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for i32 {}
    unsafe impl Reflected for i64 {
        const TYPE: Type = Type::new_prim::<i64>();
        type Key = i64;
        fn name() -> String {
            "i64".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for i64 {}
    unsafe impl Reflected for i128 {
        const TYPE: Type = Type::new_prim::<i128>();
        type Key = i128;
        fn name() -> String {
            "i128".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for i128 {}
    unsafe impl Reflected for isize {
        const TYPE: Type = Type::new_prim::<isize>();
        type Key = isize;
        fn name() -> String {
            "isize".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for isize {}
    unsafe impl Reflected for f32 {
        const TYPE: Type = Type::new_prim::<f32>();
        type Key = f32;
        fn name() -> String {
            "f32".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for f32 {}
    unsafe impl Reflected for f64 {
        const TYPE: Type = Type::new_prim::<f64>();
        type Key = f64;
        fn name() -> String {
            "f64".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for f64 {}
    unsafe impl Reflected for bool {
        const TYPE: Type = Type::new_prim::<bool>();
        type Key = bool;
        fn name() -> String {
            "bool".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for bool {}
    unsafe impl Reflected for char {
        const TYPE: Type = Type::new_prim::<char>();
        type Key = char;
        fn name() -> String {
            "char".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for char {}
    unsafe impl Reflected for str {
        const TYPE: Type = Type::new_prim::<str>();
        type Key = str;
        fn name() -> String {
            "str".into()
        }
    }
    unsafe impl<'a> NotOutlives<'a> for str {}
    impl ReflectedImpl<0> for u8 {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut, unused_unsafe)]
                        let call = |mut args: ::std::vec::Vec<crate::Value<'_>>| {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::from_str_radix(
                                    args.remove(0).cast_unsafe::<&str>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "from_str_radix";
                        let assoc_ty = crate::Type::of::<u8>();
                        let args = &[
                            crate::Type::of::<&str>(),
                            crate::Type::of::<u32>(),
                        ];
                        let ret = crate::Type::of::<
                            Result<u8, core::num::ParseIntError>,
                        >();
                        unsafe {
                            crate::AssocFn::new_static(call, name, assoc_ty, args, ret)
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::count_ones(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "count_ones";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u32>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::count_zeros(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "count_zeros";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u32>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::leading_zeros(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "leading_zeros";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u32>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::trailing_zeros(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "trailing_zeros";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u32>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::leading_ones(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "leading_ones";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u32>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::trailing_ones(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "trailing_ones";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u32>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::rotate_left(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "rotate_left";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::rotate_right(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "rotate_right";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::swap_bytes(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "swap_bytes";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::reverse_bits(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "reverse_bits";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut, unused_unsafe)]
                        let call = |mut args: ::std::vec::Vec<crate::Value<'_>>| {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::from_be(args.remove(0).cast_unsafe::<u8>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "from_be";
                        let assoc_ty = crate::Type::of::<u8>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_static(call, name, assoc_ty, args, ret)
                        }
                    },
                    {
                        #[allow(unused_mut, unused_unsafe)]
                        let call = |mut args: ::std::vec::Vec<crate::Value<'_>>| {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::from_le(args.remove(0).cast_unsafe::<u8>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "from_le";
                        let assoc_ty = crate::Type::of::<u8>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_static(call, name, assoc_ty, args, ret)
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::to_be(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_be";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::to_le(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_le";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_add(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_add";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_sub(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_sub";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_mul(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_mul";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_div(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_div";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_div_euclid(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_div_euclid";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_rem(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_rem";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_rem_euclid(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_rem_euclid";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_neg(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_neg";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_shl(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_shl";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_shr(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_shr";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_pow(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_pow";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::saturating_add(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "saturating_add";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::saturating_sub(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "saturating_sub";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::saturating_mul(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "saturating_mul";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::saturating_pow(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "saturating_pow";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_add(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_add";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_sub(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_sub";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_mul(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_mul";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_div(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_div";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_div_euclid(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_div_euclid";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_rem(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_rem";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_rem_euclid(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_rem_euclid";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_neg(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_neg";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_shl(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_shl";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_shr(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_shr";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::wrapping_pow(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_pow";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_add(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_add";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_sub(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_sub";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_mul(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_mul";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_div(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_div";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_div_euclid(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_div_euclid";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_rem(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_rem";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_rem_euclid(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_rem_euclid";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_neg(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_neg";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_shl(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_shl";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_shr(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_shr";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::overflowing_pow(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "overflowing_pow";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<(u8, bool)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::pow(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "pow";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::div_euclid(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "div_euclid";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::rem_euclid(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "rem_euclid";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u8>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_power_of_two(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_power_of_two";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::next_power_of_two(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "next_power_of_two";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::checked_next_power_of_two(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "checked_next_power_of_two";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Option<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::to_be_bytes(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_be_bytes";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<[u8; 1]>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::to_le_bytes(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_le_bytes";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<[u8; 1]>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::to_ne_bytes(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_ne_bytes";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<[u8; 1]>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut, unused_unsafe)]
                        let call = |mut args: ::std::vec::Vec<crate::Value<'_>>| {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::from_be_bytes(args.remove(0).cast_unsafe::<[u8; 1]>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "from_be_bytes";
                        let assoc_ty = crate::Type::of::<u8>();
                        let args = &[crate::Type::of::<[u8; 1]>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_static(call, name, assoc_ty, args, ret)
                        }
                    },
                    {
                        #[allow(unused_mut, unused_unsafe)]
                        let call = |mut args: ::std::vec::Vec<crate::Value<'_>>| {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::from_le_bytes(args.remove(0).cast_unsafe::<[u8; 1]>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "from_le_bytes";
                        let assoc_ty = crate::Type::of::<u8>();
                        let args = &[crate::Type::of::<[u8; 1]>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_static(call, name, assoc_ty, args, ret)
                        }
                    },
                    {
                        #[allow(unused_mut, unused_unsafe)]
                        let call = |mut args: ::std::vec::Vec<crate::Value<'_>>| {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::from_ne_bytes(args.remove(0).cast_unsafe::<[u8; 1]>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "from_ne_bytes";
                        let assoc_ty = crate::Type::of::<u8>();
                        let args = &[crate::Type::of::<[u8; 1]>()];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_static(call, name, assoc_ty, args, ret)
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::to_ascii_uppercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_ascii_uppercase";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::to_ascii_lowercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_ascii_lowercase";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::eq_ignore_ascii_case(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<&u8>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "eq_ignore_ascii_case";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<&u8>()];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::make_ascii_uppercase(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "make_ascii_uppercase";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::make_ascii_lowercase(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "make_ascii_lowercase";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii_alphabetic(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_alphabetic";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii_uppercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_uppercase";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii_lowercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_lowercase";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii_alphanumeric(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_alphanumeric";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii_digit(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_digit";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii_hexdigit(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_hexdigit";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii_punctuation(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_punctuation";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii_graphic(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_graphic";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii_whitespace(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_whitespace";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <u8>::is_ascii_control(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_control";
                        let assoc_ty = crate::Type::of::<u8>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        let ptr: ::std::boxed::Box<fn() -> _> = ::std::boxed::Box::new(||
                        {
                            use ::core::convert::From;
                            crate::Value::from(<u8>::MIN)
                        });
                        let name = "MIN";
                        let assoc_ty = crate::Type::of::<u8>();
                        let ty = crate::Type::of::<u8>();
                        unsafe { crate::AssocConst::new(ptr, name, assoc_ty, ty) }
                    },
                    {
                        let ptr: ::std::boxed::Box<fn() -> _> = ::std::boxed::Box::new(||
                        {
                            use ::core::convert::From;
                            crate::Value::from(<u8>::MAX)
                        });
                        let name = "MAX";
                        let assoc_ty = crate::Type::of::<u8>();
                        let ty = crate::Type::of::<u8>();
                        unsafe { crate::AssocConst::new(ptr, name, assoc_ty, ty) }
                    },
                ]),
            )
        }
    }
    impl ReflectedImpl<0> for bool {
        fn assoc_fns() -> Vec<AssocFn> {
            ::alloc::vec::Vec::new()
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl ReflectedImpl<0> for char {
        fn assoc_fns() -> Vec<AssocFn> {
            #[cfg(feature = "core")]
            use core::char::{
                EscapeDebug, EscapeDefault, EscapeUnicode, ToLowercase, ToUppercase,
            };
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_digit(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_digit";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::to_digit(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<u32>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_digit";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<u32>()];
                        let ret = crate::Type::of::<Option<u32>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::escape_unicode(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "escape_unicode";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<EscapeUnicode>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::escape_debug(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "escape_debug";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<EscapeDebug>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::escape_default(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "escape_default";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<EscapeDefault>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::len_utf8(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "len_utf8";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<usize>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::len_utf16(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "len_utf16";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<usize>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::encode_utf8(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<&mut [u8]>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "encode_utf8";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<&mut [u8]>()];
                        let ret = crate::Type::of::<&mut str>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::encode_utf16(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<&mut [u16]>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "encode_utf16";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<&mut [u16]>()];
                        let ret = crate::Type::of::<&mut [u16]>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_alphabetic(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_alphabetic";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_lowercase(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_lowercase";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_uppercase(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_uppercase";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_whitespace(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_whitespace";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_alphanumeric(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_alphanumeric";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_control(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_control";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_numeric(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_numeric";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::to_lowercase(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_lowercase";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<ToLowercase>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::to_uppercase(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_uppercase";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<ToUppercase>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::to_ascii_uppercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_ascii_uppercase";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<char>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::to_ascii_lowercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_ascii_lowercase";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<char>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::eq_ignore_ascii_case(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<&char>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "eq_ignore_ascii_case";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<&char>()];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::make_ascii_uppercase(
                                    this.cast_unsafe::<&mut Self>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "make_ascii_uppercase";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::make_ascii_lowercase(
                                    this.cast_unsafe::<&mut Self>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "make_ascii_lowercase";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii_alphabetic(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_alphabetic";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii_uppercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_uppercase";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii_lowercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_lowercase";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii_alphanumeric(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_alphanumeric";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii_digit(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_digit";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii_hexdigit(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_hexdigit";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii_punctuation(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_punctuation";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii_graphic(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_graphic";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii_whitespace(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_whitespace";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <char>::is_ascii_control(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii_control";
                        let assoc_ty = crate::Type::of::<char>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl ReflectedImpl<0> for str {
        fn assoc_fns() -> Vec<AssocFn> {
            #[cfg(feature = "core")]
            use core::str::{
                Bytes, CharIndices, Chars, EncodeUtf16, EscapeDebug, EscapeDefault,
                EscapeUnicode, Lines, SplitAsciiWhitespace, SplitWhitespace,
            };
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::len(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "len";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<usize>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::is_empty(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_empty";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::is_char_boundary(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_char_boundary";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::as_bytes(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "as_bytes";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<&[u8]>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::as_bytes_mut(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "as_bytes_mut";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<&mut [u8]>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::as_ptr(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "as_ptr";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<*const u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::as_mut_ptr(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "as_mut_ptr";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<*mut u8>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::split_at(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "split_at";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<(&str, &str)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::split_at_mut(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "split_at_mut";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<(&mut str, &mut str)>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::chars(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "chars";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Chars<'_>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::char_indices(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "char_indices";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<CharIndices<'_>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::bytes(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "bytes";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Bytes<'_>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::split_whitespace(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "split_whitespace";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<SplitWhitespace<'_>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::split_ascii_whitespace(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "split_ascii_whitespace";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<SplitAsciiWhitespace<'_>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::lines(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "lines";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Lines<'_>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::encode_utf16(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "encode_utf16";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<EncodeUtf16<'_>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::trim(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "trim";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<&str>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::trim_start(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "trim_start";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<&str>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::trim_end(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "trim_end";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<&str>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::is_ascii(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::eq_ignore_ascii_case(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<&str>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "eq_ignore_ascii_case";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<&str>()];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::make_ascii_uppercase(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "make_ascii_uppercase";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::make_ascii_lowercase(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "make_ascii_lowercase";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::escape_debug(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "escape_debug";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<EscapeDebug<'_>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::escape_default(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "escape_default";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<EscapeDefault<'_>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::escape_unicode(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "escape_unicode";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<EscapeUnicode<'_>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::into_boxed_bytes(this.cast_unsafe::<Box<str>>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "into_boxed_bytes";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<Box<str>>();
                        let args = &[];
                        let ret = crate::Type::of::<Box<[u8]>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::to_lowercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_lowercase";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<String>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::to_uppercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_uppercase";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<String>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::into_string(this.cast_unsafe::<Box<str>>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "into_string";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<Box<str>>();
                        let args = &[];
                        let ret = crate::Type::of::<String>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::repeat(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "repeat";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<String>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::to_ascii_uppercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_ascii_uppercase";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<String>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <str>::to_ascii_lowercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_ascii_lowercase";
                        let assoc_ty = crate::Type::of::<str>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<String>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    unsafe impl Reflected for () {
        const TYPE: Type = Type::new_tuple::<()>();
        type Key = ();
        fn name() -> String {
            "()".into()
        }
    }
    impl ReflectedTuple for () {
        const FIELDS: &'static [Field] = &[];
    }
    unsafe impl<'a> NotOutlives<'a> for () {}
    unsafe impl<TupleTy0> Reflected for (TupleTy0,)
    where
        TupleTy0: Reflected,
        TupleTy0::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (TupleTy0::Key,);
        fn name() -> String {
            let names = [TupleTy0::name()];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<TupleTy0> ReflectedTuple for (TupleTy0,)
    where
        TupleTy0: Reflected,
        TupleTy0::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
            ]
        };
    }
    unsafe impl<TupleTy0, TupleTy1> Reflected for (TupleTy0, TupleTy1)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (TupleTy0::Key, TupleTy1::Key);
        fn name() -> String {
            let names = [TupleTy0::name(), TupleTy1::name()];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<TupleTy0, TupleTy1> ReflectedTuple for (TupleTy0, TupleTy1)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
            ]
        };
    }
    unsafe impl<TupleTy0, TupleTy1, TupleTy2> Reflected
    for (TupleTy0, TupleTy1, TupleTy2)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (TupleTy0::Key, TupleTy1::Key, TupleTy2::Key);
        fn name() -> String {
            let names = [TupleTy0::name(), TupleTy1::name(), TupleTy2::name()];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<TupleTy0, TupleTy1, TupleTy2> ReflectedTuple for (TupleTy0, TupleTy1, TupleTy2)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
            ]
        };
    }
    unsafe impl<TupleTy0, TupleTy1, TupleTy2, TupleTy3> Reflected
    for (TupleTy0, TupleTy1, TupleTy2, TupleTy3)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (TupleTy0::Key, TupleTy1::Key, TupleTy2::Key, TupleTy3::Key);
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<TupleTy0, TupleTy1, TupleTy2, TupleTy3> ReflectedTuple
    for (TupleTy0, TupleTy1, TupleTy2, TupleTy3)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
            ]
        };
    }
    unsafe impl<TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4> Reflected
    for (TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4> ReflectedTuple
    for (TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
            ]
        };
    }
    unsafe impl<TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4, TupleTy5> Reflected
    for (TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4, TupleTy5)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4, TupleTy5> ReflectedTuple
    for (TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4, TupleTy5)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
    > Reflected
    for (TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4, TupleTy5, TupleTy6)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
    > ReflectedTuple
    for (TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4, TupleTy5, TupleTy6)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
    > Reflected
    for (TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4, TupleTy5, TupleTy6, TupleTy7)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
    > ReflectedTuple
    for (TupleTy0, TupleTy1, TupleTy2, TupleTy3, TupleTy4, TupleTy5, TupleTy6, TupleTy7)
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
            TupleTy16::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
                TupleTy16::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.16);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.16 = unsafe { value.cast_unsafe::<TupleTy16>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 16, Self::TYPE, TupleTy16::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
            TupleTy16::Key,
            TupleTy17::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
                TupleTy16::name(),
                TupleTy17::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.16);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.16 = unsafe { value.cast_unsafe::<TupleTy16>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 16, Self::TYPE, TupleTy16::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.17);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.17 = unsafe { value.cast_unsafe::<TupleTy17>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 17, Self::TYPE, TupleTy17::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
            TupleTy16::Key,
            TupleTy17::Key,
            TupleTy18::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
                TupleTy16::name(),
                TupleTy17::name(),
                TupleTy18::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.16);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.16 = unsafe { value.cast_unsafe::<TupleTy16>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 16, Self::TYPE, TupleTy16::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.17);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.17 = unsafe { value.cast_unsafe::<TupleTy17>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 17, Self::TYPE, TupleTy17::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.18);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.18 = unsafe { value.cast_unsafe::<TupleTy18>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 18, Self::TYPE, TupleTy18::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
            TupleTy16::Key,
            TupleTy17::Key,
            TupleTy18::Key,
            TupleTy19::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
                TupleTy16::name(),
                TupleTy17::name(),
                TupleTy18::name(),
                TupleTy19::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.16);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.16 = unsafe { value.cast_unsafe::<TupleTy16>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 16, Self::TYPE, TupleTy16::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.17);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.17 = unsafe { value.cast_unsafe::<TupleTy17>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 17, Self::TYPE, TupleTy17::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.18);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.18 = unsafe { value.cast_unsafe::<TupleTy18>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 18, Self::TYPE, TupleTy18::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.19);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.19 = unsafe { value.cast_unsafe::<TupleTy19>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 19, Self::TYPE, TupleTy19::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
            TupleTy16::Key,
            TupleTy17::Key,
            TupleTy18::Key,
            TupleTy19::Key,
            TupleTy20::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
                TupleTy16::name(),
                TupleTy17::name(),
                TupleTy18::name(),
                TupleTy19::name(),
                TupleTy20::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.16);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.16 = unsafe { value.cast_unsafe::<TupleTy16>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 16, Self::TYPE, TupleTy16::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.17);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.17 = unsafe { value.cast_unsafe::<TupleTy17>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 17, Self::TYPE, TupleTy17::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.18);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.18 = unsafe { value.cast_unsafe::<TupleTy18>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 18, Self::TYPE, TupleTy18::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.19);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.19 = unsafe { value.cast_unsafe::<TupleTy19>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 19, Self::TYPE, TupleTy19::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.20);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.20 = unsafe { value.cast_unsafe::<TupleTy20>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 20, Self::TYPE, TupleTy20::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy21: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
        TupleTy21::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
            TupleTy16::Key,
            TupleTy17::Key,
            TupleTy18::Key,
            TupleTy19::Key,
            TupleTy20::Key,
            TupleTy21::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
                TupleTy16::name(),
                TupleTy17::name(),
                TupleTy18::name(),
                TupleTy19::name(),
                TupleTy20::name(),
                TupleTy21::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy21: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
        TupleTy21::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.16);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.16 = unsafe { value.cast_unsafe::<TupleTy16>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 16, Self::TYPE, TupleTy16::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.17);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.17 = unsafe { value.cast_unsafe::<TupleTy17>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 17, Self::TYPE, TupleTy17::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.18);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.18 = unsafe { value.cast_unsafe::<TupleTy18>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 18, Self::TYPE, TupleTy18::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.19);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.19 = unsafe { value.cast_unsafe::<TupleTy19>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 19, Self::TYPE, TupleTy19::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.20);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.20 = unsafe { value.cast_unsafe::<TupleTy20>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 20, Self::TYPE, TupleTy20::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.21);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.21 = unsafe { value.cast_unsafe::<TupleTy21>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 21, Self::TYPE, TupleTy21::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy21: Reflected,
        TupleTy22: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
        TupleTy21::Key: Sized,
        TupleTy22::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
            TupleTy16::Key,
            TupleTy17::Key,
            TupleTy18::Key,
            TupleTy19::Key,
            TupleTy20::Key,
            TupleTy21::Key,
            TupleTy22::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
                TupleTy16::name(),
                TupleTy17::name(),
                TupleTy18::name(),
                TupleTy19::name(),
                TupleTy20::name(),
                TupleTy21::name(),
                TupleTy22::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy21: Reflected,
        TupleTy22: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
        TupleTy21::Key: Sized,
        TupleTy22::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.16);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.16 = unsafe { value.cast_unsafe::<TupleTy16>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 16, Self::TYPE, TupleTy16::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.17);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.17 = unsafe { value.cast_unsafe::<TupleTy17>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 17, Self::TYPE, TupleTy17::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.18);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.18 = unsafe { value.cast_unsafe::<TupleTy18>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 18, Self::TYPE, TupleTy18::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.19);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.19 = unsafe { value.cast_unsafe::<TupleTy19>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 19, Self::TYPE, TupleTy19::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.20);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.20 = unsafe { value.cast_unsafe::<TupleTy20>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 20, Self::TYPE, TupleTy20::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.21);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.21 = unsafe { value.cast_unsafe::<TupleTy21>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 21, Self::TYPE, TupleTy21::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.22);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.22 = unsafe { value.cast_unsafe::<TupleTy22>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 22, Self::TYPE, TupleTy22::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy21: Reflected,
        TupleTy22: Reflected,
        TupleTy23: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
        TupleTy21::Key: Sized,
        TupleTy22::Key: Sized,
        TupleTy23::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
            TupleTy16::Key,
            TupleTy17::Key,
            TupleTy18::Key,
            TupleTy19::Key,
            TupleTy20::Key,
            TupleTy21::Key,
            TupleTy22::Key,
            TupleTy23::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
                TupleTy16::name(),
                TupleTy17::name(),
                TupleTy18::name(),
                TupleTy19::name(),
                TupleTy20::name(),
                TupleTy21::name(),
                TupleTy22::name(),
                TupleTy23::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy21: Reflected,
        TupleTy22: Reflected,
        TupleTy23: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
        TupleTy21::Key: Sized,
        TupleTy22::Key: Sized,
        TupleTy23::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.16);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.16 = unsafe { value.cast_unsafe::<TupleTy16>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 16, Self::TYPE, TupleTy16::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.17);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.17 = unsafe { value.cast_unsafe::<TupleTy17>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 17, Self::TYPE, TupleTy17::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.18);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.18 = unsafe { value.cast_unsafe::<TupleTy18>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 18, Self::TYPE, TupleTy18::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.19);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.19 = unsafe { value.cast_unsafe::<TupleTy19>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 19, Self::TYPE, TupleTy19::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.20);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.20 = unsafe { value.cast_unsafe::<TupleTy20>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 20, Self::TYPE, TupleTy20::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.21);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.21 = unsafe { value.cast_unsafe::<TupleTy21>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 21, Self::TYPE, TupleTy21::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.22);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.22 = unsafe { value.cast_unsafe::<TupleTy22>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 22, Self::TYPE, TupleTy22::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.23);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.23 = unsafe { value.cast_unsafe::<TupleTy23>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 23, Self::TYPE, TupleTy23::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
        TupleTy24,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
        TupleTy24,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy21: Reflected,
        TupleTy22: Reflected,
        TupleTy23: Reflected,
        TupleTy24: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
        TupleTy21::Key: Sized,
        TupleTy22::Key: Sized,
        TupleTy23::Key: Sized,
        TupleTy24::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
            TupleTy16::Key,
            TupleTy17::Key,
            TupleTy18::Key,
            TupleTy19::Key,
            TupleTy20::Key,
            TupleTy21::Key,
            TupleTy22::Key,
            TupleTy23::Key,
            TupleTy24::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
                TupleTy16::name(),
                TupleTy17::name(),
                TupleTy18::name(),
                TupleTy19::name(),
                TupleTy20::name(),
                TupleTy21::name(),
                TupleTy22::name(),
                TupleTy23::name(),
                TupleTy24::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
        TupleTy24,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
        TupleTy24,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy21: Reflected,
        TupleTy22: Reflected,
        TupleTy23: Reflected,
        TupleTy24: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
        TupleTy21::Key: Sized,
        TupleTy22::Key: Sized,
        TupleTy23::Key: Sized,
        TupleTy24::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.16);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.16 = unsafe { value.cast_unsafe::<TupleTy16>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 16, Self::TYPE, TupleTy16::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.17);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.17 = unsafe { value.cast_unsafe::<TupleTy17>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 17, Self::TYPE, TupleTy17::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.18);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.18 = unsafe { value.cast_unsafe::<TupleTy18>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 18, Self::TYPE, TupleTy18::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.19);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.19 = unsafe { value.cast_unsafe::<TupleTy19>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 19, Self::TYPE, TupleTy19::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.20);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.20 = unsafe { value.cast_unsafe::<TupleTy20>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 20, Self::TYPE, TupleTy20::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.21);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.21 = unsafe { value.cast_unsafe::<TupleTy21>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 21, Self::TYPE, TupleTy21::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.22);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.22 = unsafe { value.cast_unsafe::<TupleTy22>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 22, Self::TYPE, TupleTy22::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.23);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.23 = unsafe { value.cast_unsafe::<TupleTy23>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 23, Self::TYPE, TupleTy23::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.24);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.24 = unsafe { value.cast_unsafe::<TupleTy24>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 24, Self::TYPE, TupleTy24::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
        TupleTy24,
        TupleTy25,
    > Reflected
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
        TupleTy24,
        TupleTy25,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy21: Reflected,
        TupleTy22: Reflected,
        TupleTy23: Reflected,
        TupleTy24: Reflected,
        TupleTy25: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
        TupleTy21::Key: Sized,
        TupleTy22::Key: Sized,
        TupleTy23::Key: Sized,
        TupleTy24::Key: Sized,
        TupleTy25::Key: Sized,
    {
        const TYPE: Type = Type::new_tuple::<Self>();
        type Key = (
            TupleTy0::Key,
            TupleTy1::Key,
            TupleTy2::Key,
            TupleTy3::Key,
            TupleTy4::Key,
            TupleTy5::Key,
            TupleTy6::Key,
            TupleTy7::Key,
            TupleTy8::Key,
            TupleTy9::Key,
            TupleTy10::Key,
            TupleTy11::Key,
            TupleTy12::Key,
            TupleTy13::Key,
            TupleTy14::Key,
            TupleTy15::Key,
            TupleTy16::Key,
            TupleTy17::Key,
            TupleTy18::Key,
            TupleTy19::Key,
            TupleTy20::Key,
            TupleTy21::Key,
            TupleTy22::Key,
            TupleTy23::Key,
            TupleTy24::Key,
            TupleTy25::Key,
        );
        fn name() -> String {
            let names = [
                TupleTy0::name(),
                TupleTy1::name(),
                TupleTy2::name(),
                TupleTy3::name(),
                TupleTy4::name(),
                TupleTy5::name(),
                TupleTy6::name(),
                TupleTy7::name(),
                TupleTy8::name(),
                TupleTy9::name(),
                TupleTy10::name(),
                TupleTy11::name(),
                TupleTy12::name(),
                TupleTy13::name(),
                TupleTy14::name(),
                TupleTy15::name(),
                TupleTy16::name(),
                TupleTy17::name(),
                TupleTy18::name(),
                TupleTy19::name(),
                TupleTy20::name(),
                TupleTy21::name(),
                TupleTy22::name(),
                TupleTy23::name(),
                TupleTy24::name(),
                TupleTy25::name(),
            ];
            {
                let res = ::alloc::fmt::format(format_args!("({0})", names.join(", ")));
                res
            }
        }
    }
    impl<
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
        TupleTy24,
        TupleTy25,
    > ReflectedTuple
    for (
        TupleTy0,
        TupleTy1,
        TupleTy2,
        TupleTy3,
        TupleTy4,
        TupleTy5,
        TupleTy6,
        TupleTy7,
        TupleTy8,
        TupleTy9,
        TupleTy10,
        TupleTy11,
        TupleTy12,
        TupleTy13,
        TupleTy14,
        TupleTy15,
        TupleTy16,
        TupleTy17,
        TupleTy18,
        TupleTy19,
        TupleTy20,
        TupleTy21,
        TupleTy22,
        TupleTy23,
        TupleTy24,
        TupleTy25,
    )
    where
        TupleTy0: Reflected,
        TupleTy1: Reflected,
        TupleTy2: Reflected,
        TupleTy3: Reflected,
        TupleTy4: Reflected,
        TupleTy5: Reflected,
        TupleTy6: Reflected,
        TupleTy7: Reflected,
        TupleTy8: Reflected,
        TupleTy9: Reflected,
        TupleTy10: Reflected,
        TupleTy11: Reflected,
        TupleTy12: Reflected,
        TupleTy13: Reflected,
        TupleTy14: Reflected,
        TupleTy15: Reflected,
        TupleTy16: Reflected,
        TupleTy17: Reflected,
        TupleTy18: Reflected,
        TupleTy19: Reflected,
        TupleTy20: Reflected,
        TupleTy21: Reflected,
        TupleTy22: Reflected,
        TupleTy23: Reflected,
        TupleTy24: Reflected,
        TupleTy25: Reflected,
        TupleTy0::Key: Sized,
        TupleTy1::Key: Sized,
        TupleTy2::Key: Sized,
        TupleTy3::Key: Sized,
        TupleTy4::Key: Sized,
        TupleTy5::Key: Sized,
        TupleTy6::Key: Sized,
        TupleTy7::Key: Sized,
        TupleTy8::Key: Sized,
        TupleTy9::Key: Sized,
        TupleTy10::Key: Sized,
        TupleTy11::Key: Sized,
        TupleTy12::Key: Sized,
        TupleTy13::Key: Sized,
        TupleTy14::Key: Sized,
        TupleTy15::Key: Sized,
        TupleTy16::Key: Sized,
        TupleTy17::Key: Sized,
        TupleTy18::Key: Sized,
        TupleTy19::Key: Sized,
        TupleTy20::Key: Sized,
        TupleTy21::Key: Sized,
        TupleTy22::Key: Sized,
        TupleTy23::Key: Sized,
        TupleTy24::Key: Sized,
        TupleTy25::Key: Sized,
    {
        const FIELDS: &'static [Field] = {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;
            &[
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.0);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.0 = unsafe { value.cast_unsafe::<TupleTy0>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 0, Self::TYPE, TupleTy0::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.1);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.1 = unsafe { value.cast_unsafe::<TupleTy1>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 1, Self::TYPE, TupleTy1::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.2);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.2 = unsafe { value.cast_unsafe::<TupleTy2>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 2, Self::TYPE, TupleTy2::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.3);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.3 = unsafe { value.cast_unsafe::<TupleTy3>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 3, Self::TYPE, TupleTy3::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.4);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.4 = unsafe { value.cast_unsafe::<TupleTy4>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 4, Self::TYPE, TupleTy4::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.5);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.5 = unsafe { value.cast_unsafe::<TupleTy5>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 5, Self::TYPE, TupleTy5::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.6);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.6 = unsafe { value.cast_unsafe::<TupleTy6>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 6, Self::TYPE, TupleTy6::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.7);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.7 = unsafe { value.cast_unsafe::<TupleTy7>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 7, Self::TYPE, TupleTy7::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.8);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.8 = unsafe { value.cast_unsafe::<TupleTy8>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 8, Self::TYPE, TupleTy8::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.9);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.9 = unsafe { value.cast_unsafe::<TupleTy9>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 9, Self::TYPE, TupleTy9::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.10);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.10 = unsafe { value.cast_unsafe::<TupleTy10>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 10, Self::TYPE, TupleTy10::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.11);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.11 = unsafe { value.cast_unsafe::<TupleTy11>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 11, Self::TYPE, TupleTy11::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.12);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.12 = unsafe { value.cast_unsafe::<TupleTy12>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 12, Self::TYPE, TupleTy12::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.13);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.13 = unsafe { value.cast_unsafe::<TupleTy13>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 13, Self::TYPE, TupleTy13::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.14);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.14 = unsafe { value.cast_unsafe::<TupleTy14>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 14, Self::TYPE, TupleTy14::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.15);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.15 = unsafe { value.cast_unsafe::<TupleTy15>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 15, Self::TYPE, TupleTy15::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.16);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.16 = unsafe { value.cast_unsafe::<TupleTy16>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 16, Self::TYPE, TupleTy16::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.17);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.17 = unsafe { value.cast_unsafe::<TupleTy17>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 17, Self::TYPE, TupleTy17::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.18);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.18 = unsafe { value.cast_unsafe::<TupleTy18>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 18, Self::TYPE, TupleTy18::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.19);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.19 = unsafe { value.cast_unsafe::<TupleTy19>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 19, Self::TYPE, TupleTy19::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.20);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.20 = unsafe { value.cast_unsafe::<TupleTy20>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 20, Self::TYPE, TupleTy20::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.21);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.21 = unsafe { value.cast_unsafe::<TupleTy21>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 21, Self::TYPE, TupleTy21::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.22);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.22 = unsafe { value.cast_unsafe::<TupleTy22>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 22, Self::TYPE, TupleTy22::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.23);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.23 = unsafe { value.cast_unsafe::<TupleTy23>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 23, Self::TYPE, TupleTy23::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.24);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.24 = unsafe { value.cast_unsafe::<TupleTy24>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 24, Self::TYPE, TupleTy24::TYPE)
                },
                {
                    let get_ptr: Option<AccessHelper> = Some(|this| {
                        let inner = unsafe { this.borrow_unsafe::<Self>() };
                        let v = Value::from_ref(&inner.25);
                        unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                    });
                    let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                        let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                        inner.25 = unsafe { value.cast_unsafe::<TupleTy25>() };
                    });
                    Field::new_tuple(get_ptr, set_ptr, 25, Self::TYPE, TupleTy25::TYPE)
                },
            ]
        };
    }
    unsafe impl<
        'no,
        'a,
        'b,
        'c,
        'd,
        'e,
        'f,
        'g,
        'h,
        'i,
        'j,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no>
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'a + 'b + 'c + 'd + 'e + 'f + 'g + 'h + 'i + 'j + 'k + 'l + 'm + 'n + 'o
            + 'p + 'q + 'r + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        A: NotOutlives<'a>,
        B: NotOutlives<'b>,
        C: NotOutlives<'c>,
        D: NotOutlives<'d>,
        E: NotOutlives<'e>,
        F: NotOutlives<'f>,
        G: NotOutlives<'g>,
        H: NotOutlives<'h>,
        I: NotOutlives<'i>,
        J: NotOutlives<'j>,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'b,
        'c,
        'd,
        'e,
        'f,
        'g,
        'h,
        'i,
        'j,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no>
    for (B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'b + 'c + 'd + 'e + 'f + 'g + 'h + 'i + 'j + 'k + 'l + 'm + 'n + 'o + 'p
            + 'q + 'r + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        B: NotOutlives<'b>,
        C: NotOutlives<'c>,
        D: NotOutlives<'d>,
        E: NotOutlives<'e>,
        F: NotOutlives<'f>,
        G: NotOutlives<'g>,
        H: NotOutlives<'h>,
        I: NotOutlives<'i>,
        J: NotOutlives<'j>,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'c,
        'd,
        'e,
        'f,
        'g,
        'h,
        'i,
        'j,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no>
    for (C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'c + 'd + 'e + 'f + 'g + 'h + 'i + 'j + 'k + 'l + 'm + 'n + 'o + 'p + 'q
            + 'r + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        C: NotOutlives<'c>,
        D: NotOutlives<'d>,
        E: NotOutlives<'e>,
        F: NotOutlives<'f>,
        G: NotOutlives<'g>,
        H: NotOutlives<'h>,
        I: NotOutlives<'i>,
        J: NotOutlives<'j>,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'd,
        'e,
        'f,
        'g,
        'h,
        'i,
        'j,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no>
    for (D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'd + 'e + 'f + 'g + 'h + 'i + 'j + 'k + 'l + 'm + 'n + 'o + 'p + 'q + 'r
            + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        D: NotOutlives<'d>,
        E: NotOutlives<'e>,
        F: NotOutlives<'f>,
        G: NotOutlives<'g>,
        H: NotOutlives<'h>,
        I: NotOutlives<'i>,
        J: NotOutlives<'j>,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'e,
        'f,
        'g,
        'h,
        'i,
        'j,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no>
    for (E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'e + 'f + 'g + 'h + 'i + 'j + 'k + 'l + 'm + 'n + 'o + 'p + 'q + 'r + 's
            + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        E: NotOutlives<'e>,
        F: NotOutlives<'f>,
        G: NotOutlives<'g>,
        H: NotOutlives<'h>,
        I: NotOutlives<'i>,
        J: NotOutlives<'j>,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'f,
        'g,
        'h,
        'i,
        'j,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no>
    for (F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'f + 'g + 'h + 'i + 'j + 'k + 'l + 'm + 'n + 'o + 'p + 'q + 'r + 's + 't
            + 'u + 'v + 'w + 'x + 'y + 'z,
        F: NotOutlives<'f>,
        G: NotOutlives<'g>,
        H: NotOutlives<'h>,
        I: NotOutlives<'i>,
        J: NotOutlives<'j>,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'g,
        'h,
        'i,
        'j,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'g + 'h + 'i + 'j + 'k + 'l + 'm + 'n + 'o + 'p + 'q + 'r + 's + 't + 'u
            + 'v + 'w + 'x + 'y + 'z,
        G: NotOutlives<'g>,
        H: NotOutlives<'h>,
        I: NotOutlives<'i>,
        J: NotOutlives<'j>,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'h,
        'i,
        'j,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'h + 'i + 'j + 'k + 'l + 'm + 'n + 'o + 'p + 'q + 'r + 's + 't + 'u + 'v
            + 'w + 'x + 'y + 'z,
        H: NotOutlives<'h>,
        I: NotOutlives<'i>,
        J: NotOutlives<'j>,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'i,
        'j,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'i + 'j + 'k + 'l + 'm + 'n + 'o + 'p + 'q + 'r + 's + 't + 'u + 'v + 'w
            + 'x + 'y + 'z,
        I: NotOutlives<'i>,
        J: NotOutlives<'j>,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'j,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'j + 'k + 'l + 'm + 'n + 'o + 'p + 'q + 'r + 's + 't + 'u + 'v + 'w + 'x
            + 'y + 'z,
        J: NotOutlives<'j>,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'k,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'k + 'l + 'm + 'n + 'o + 'p + 'q + 'r + 's + 't + 'u + 'v + 'w + 'x + 'y
            + 'z,
        K: NotOutlives<'k>,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'l,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'l + 'm + 'n + 'o + 'p + 'q + 'r + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        L: NotOutlives<'l>,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'm,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'm + 'n + 'o + 'p + 'q + 'r + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        M: NotOutlives<'m>,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'n,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'n + 'o + 'p + 'q + 'r + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        N: NotOutlives<'n>,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'o,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (O, P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'o + 'p + 'q + 'r + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        O: NotOutlives<'o>,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'p,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (P, Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'p + 'q + 'r + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        P: NotOutlives<'p>,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'q,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (Q, R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'q + 'r + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        Q: NotOutlives<'q>,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        'r,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (R, S, T, U, V, W, X, Y, Z)
    where
        'no: 'r + 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        R: NotOutlives<'r>,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<
        'no,
        's,
        't,
        'u,
        'v,
        'w,
        'x,
        'y,
        'z,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > NotOutlives<'no> for (S, T, U, V, W, X, Y, Z)
    where
        'no: 's + 't + 'u + 'v + 'w + 'x + 'y + 'z,
        S: NotOutlives<'s>,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<'no, 't, 'u, 'v, 'w, 'x, 'y, 'z, T, U, V, W, X, Y, Z> NotOutlives<'no>
    for (T, U, V, W, X, Y, Z)
    where
        'no: 't + 'u + 'v + 'w + 'x + 'y + 'z,
        T: NotOutlives<'t>,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<'no, 'u, 'v, 'w, 'x, 'y, 'z, U, V, W, X, Y, Z> NotOutlives<'no>
    for (U, V, W, X, Y, Z)
    where
        'no: 'u + 'v + 'w + 'x + 'y + 'z,
        U: NotOutlives<'u>,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<'no, 'v, 'w, 'x, 'y, 'z, V, W, X, Y, Z> NotOutlives<'no>
    for (V, W, X, Y, Z)
    where
        'no: 'v + 'w + 'x + 'y + 'z,
        V: NotOutlives<'v>,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<'no, 'w, 'x, 'y, 'z, W, X, Y, Z> NotOutlives<'no> for (W, X, Y, Z)
    where
        'no: 'w + 'x + 'y + 'z,
        W: NotOutlives<'w>,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<'no, 'x, 'y, 'z, X, Y, Z> NotOutlives<'no> for (X, Y, Z)
    where
        'no: 'x + 'y + 'z,
        X: NotOutlives<'x>,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<'no, 'y, 'z, Y, Z> NotOutlives<'no> for (Y, Z)
    where
        'no: 'y + 'z,
        Y: NotOutlives<'y>,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<'no, 'z, Z> NotOutlives<'no> for (Z,)
    where
        'no: 'z,
        Z: NotOutlives<'z>,
    {}
    unsafe impl<T, const N: usize> Reflected for [T; N]
    where
        T: Reflected,
        T::Key: Sized,
    {
        const TYPE: Type = Type::new_array::<[T; N]>();
        type Key = [T::Key; N];
        fn name() -> String {
            {
                let res = ::alloc::fmt::format(format_args!("[{0}; {1}]", T::name(), N));
                res
            }
        }
    }
    impl<T, const N: usize> ReflectedArray for [T; N]
    where
        T: Reflected,
        T::Key: Sized,
    {
        const LENGTH: usize = N;
        fn element() -> Type {
            Type::of::<T>()
        }
    }
    unsafe impl<'a, T, const N: usize> NotOutlives<'a> for [T; N]
    where
        T: NotOutlives<'a>,
    {}
    unsafe impl<T> Reflected for [T]
    where
        T: Reflected,
        T::Key: Sized,
    {
        const TYPE: Type = Type::new_slice::<[T]>();
        type Key = [T::Key];
        fn name() -> String {
            {
                let res = ::alloc::fmt::format(format_args!("[{0}]", T::name()));
                res
            }
        }
    }
    impl<T> ReflectedSlice for [T]
    where
        T: Reflected,
        T::Key: Sized,
    {
        fn element() -> Type {
            Type::of::<T>()
        }
    }
    unsafe impl<'a, 'b, T> NotOutlives<'b> for [T]
    where
        'b: 'a,
        T: NotOutlives<'a>,
    {}
    impl<T> ReflectedImpl<0> for [T]
    where
        T: Reflected,
        T::Key: Reflected + Sized,
        <T::Key as Reflected>::Key: Sized,
    {
        fn assoc_fns() -> Vec<AssocFn> {
            #[cfg(feature = "core")]
            use core::ops::Range;
            #[cfg(feature = "core")]
            use core::slice::{
                Chunks, ChunksExact, ChunksExactMut, ChunksMut, Iter, IterMut, RChunks,
                RChunksExact, RChunksExactMut, RChunksMut, Windows,
            };
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::len(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "len";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<usize>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::is_empty(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_empty";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::first(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "first";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Option<&T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::first_mut(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "first_mut";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Option<&mut T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::split_first(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "split_first";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Option<(&T, &[T])>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::split_first_mut(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "split_first_mut";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Option<(&mut T, &mut [T])>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::split_last(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "split_last";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Option<(&T, &[T])>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::split_last_mut(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "split_last_mut";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Option<(&mut T, &mut [T])>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::last(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "last";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Option<&T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::last_mut(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "last_mut";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Option<&mut T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::as_ptr(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "as_ptr";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<*const T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::as_mut_ptr(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "as_mut_ptr";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<*mut T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::as_ptr_range(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "as_ptr_range";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Range<*const T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::as_mut_ptr_range(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "as_mut_ptr_range";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Range<*mut T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::swap(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "swap";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[
                            crate::Type::of::<usize>(),
                            crate::Type::of::<usize>(),
                        ];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::reverse(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "reverse";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::iter(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "iter";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Iter<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::iter_mut(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "iter_mut";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<IterMut<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::windows(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "windows";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<Windows<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::chunks(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "chunks";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<Chunks<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::chunks_mut(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "chunks_mut";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<ChunksMut<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::chunks_exact(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "chunks_exact";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<ChunksExact<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::chunks_exact_mut(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "chunks_exact_mut";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<ChunksExactMut<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::rchunks(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "rchunks";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<RChunks<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::rchunks_mut(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "rchunks_mut";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<RChunksMut<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::rchunks_exact(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "rchunks_exact";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<RChunksExact<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::rchunks_exact_mut(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "rchunks_exact_mut";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<RChunksExactMut<'_, T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::split_at(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "split_at";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<(&[T], &[T])>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::split_at_mut(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "split_at_mut";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<(&mut [T], &mut [T])>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::rotate_left(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "rotate_left";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::rotate_right(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "rotate_right";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T> ReflectedImpl<1> for [T]
    where
        T: Reflected + PartialEq,
        T::Key: Sized,
    {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::contains(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<&T>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "contains";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<&T>()];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::starts_with(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<&[T]>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "starts_with";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<&[T]>()];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::ends_with(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<&[T]>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "ends_with";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<&[T]>()];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T> ReflectedImpl<2> for [T]
    where
        T: Reflected + PartialOrd,
        T::Key: Sized,
    {
        fn assoc_fns() -> Vec<AssocFn> {
            ::alloc::vec::Vec::new()
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T> ReflectedImpl<3> for [T]
    where
        T: Reflected + Ord,
        T::Key: Sized,
    {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::sort(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "sort";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::sort_unstable(this.cast_unsafe::<&mut Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "sort_unstable";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "core")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::binary_search(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<&T>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "binary_search";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<&T>()];
                        let ret = crate::Type::of::<Result<usize, usize>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T> ReflectedImpl<4> for [T]
    where
        T: Reflected + Clone,
        T::Key: Sized,
    {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::clone_from_slice(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<&[T]>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "clone_from_slice";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<&[T]>()];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T> ReflectedImpl<5> for [T]
    where
        T: Reflected + Clone + 'static,
        T::Key: Reflected + Sized,
        <T::Key as Reflected>::Key: Reflected + Sized,
    {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::to_vec(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_vec";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Vec<T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::into_vec(this.cast_unsafe::<Box<[T]>>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "into_vec";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<Box<[T]>>();
                        let args = &[];
                        let ret = crate::Type::of::<Vec<T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T> ReflectedImpl<6> for [T]
    where
        T: Reflected + Copy,
        T::Key: Sized,
    {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::copy_from_slice(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<&[T]>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "copy_from_slice";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<&[T]>()];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::swap_with_slice(
                                    this.cast_unsafe::<&mut Self>(),
                                    args.remove(0).cast_unsafe::<&mut [T]>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "swap_with_slice";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[crate::Type::of::<&mut [T]>()];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T> ReflectedImpl<7> for [T]
    where
        T: Reflected + Copy + 'static,
        T::Key: Reflected + Sized,
    {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[T]>::repeat(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "repeat";
                        let assoc_ty = crate::Type::of::<[T]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<Vec<T>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl ReflectedImpl<8> for [u8] {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[u8]>::is_ascii(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_ascii";
                        let assoc_ty = crate::Type::of::<[u8]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[u8]>::eq_ignore_ascii_case(
                                    this.cast_unsafe::<&Self>(),
                                    args.remove(0).cast_unsafe::<&[u8]>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "eq_ignore_ascii_case";
                        let assoc_ty = crate::Type::of::<[u8]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[crate::Type::of::<&[u8]>()];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[u8]>::make_ascii_uppercase(
                                    this.cast_unsafe::<&mut Self>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "make_ascii_uppercase";
                        let assoc_ty = crate::Type::of::<[u8]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[u8]>::make_ascii_lowercase(
                                    this.cast_unsafe::<&mut Self>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "make_ascii_lowercase";
                        let assoc_ty = crate::Type::of::<[u8]>();
                        let self_ty = crate::Type::of::<&mut Self>();
                        let args = &[];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[u8]>::to_ascii_uppercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_ascii_uppercase";
                        let assoc_ty = crate::Type::of::<[u8]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Vec<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    #[cfg(feature = "alloc")]
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <[u8]>::to_ascii_lowercase(this.cast_unsafe::<&Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "to_ascii_lowercase";
                        let assoc_ty = crate::Type::of::<[u8]>();
                        let self_ty = crate::Type::of::<&Self>();
                        let args = &[];
                        let ret = crate::Type::of::<Vec<u8>>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    unsafe impl<T: ?Sized + Reflected> Reflected for *const T {
        const TYPE: Type = Type::new_ptr::<*const T>();
        type Key = *const T::Key;
        fn name() -> String {
            {
                let res = ::alloc::fmt::format(format_args!("*const {0}", T::name()));
                res
            }
        }
    }
    impl<T: ?Sized + Reflected> ReflectedPointer for *const T {
        const MUTABILITY: bool = false;
        fn element() -> Type {
            Type::of::<T>()
        }
    }
    unsafe impl<'a, T> NotOutlives<'a> for *const T
    where
        T: ?Sized + NotOutlives<'a>,
    {}
    impl<T: ?Sized + Reflected> ReflectedImpl<0> for *const T {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::is_null(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "is_null";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<bool>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Reflected> ReflectedImpl<1> for *const T {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::offset_from(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<*const T>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "offset_from";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<*const T>()];
                        let ret = crate::Type::of::<isize>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::copy_to(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<*mut T>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "copy_to";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[
                            crate::Type::of::<*mut T>(),
                            crate::Type::of::<usize>(),
                        ];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::copy_to_nonoverlapping(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<*mut T>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "copy_to_nonoverlapping";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[
                            crate::Type::of::<*mut T>(),
                            crate::Type::of::<usize>(),
                        ];
                        let ret = crate::Type::of::<()>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::align_offset(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "align_offset";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<usize>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Reflected + 'static> ReflectedImpl<2> for *const T {
        fn assoc_fns() -> Vec<AssocFn> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::offset(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<isize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "offset";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<isize>()];
                        let ret = crate::Type::of::<*const T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::wrapping_offset(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<isize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_offset";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<isize>()];
                        let ret = crate::Type::of::<*const T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::read(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "read";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::read_volatile(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "read_volatile";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::read_unaligned(this.cast_unsafe::<Self>())
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "read_unaligned";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[];
                        let ret = crate::Type::of::<T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::add(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "add";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<*const T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::sub(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "sub";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<*const T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::wrapping_add(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_add";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<*const T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                    {
                        #[allow(unused_mut)]
                        let call = |
                            this: crate::Value<'_>,
                            mut args: ::std::vec::Vec<crate::Value<'_>>|
                        {
                            use ::core::convert::From;
                            let v = crate::Value::from(unsafe {
                                <*const T>::wrapping_sub(
                                    this.cast_unsafe::<Self>(),
                                    args.remove(0).cast_unsafe::<usize>(),
                                )
                            });
                            if true {
                                match (&args.len(), &0) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                            unsafe {
                                ::core::mem::transmute::<
                                    crate::Value<'_>,
                                    crate::Value<'_>,
                                >(v)
                            }
                        };
                        let name = "wrapping_sub";
                        let assoc_ty = crate::Type::of::<*const T>();
                        let self_ty = crate::Type::of::<Self>();
                        let args = &[crate::Type::of::<usize>()];
                        let ret = crate::Type::of::<*const T>();
                        unsafe {
                            crate::AssocFn::new_dynamic(
                                call,
                                name,
                                assoc_ty,
                                self_ty,
                                args,
                                ret,
                            )
                        }
                    },
                ]),
            )
        }
        fn assoc_consts() -> Vec<AssocConst> {
            ::alloc::vec::Vec::new()
        }
    }
    unsafe impl<T: ?Sized + Reflected> Reflected for *mut T {
        const TYPE: Type = Type::new_ptr::<*mut T>();
        type Key = *mut T::Key;
        fn name() -> String {
            {
                let res = ::alloc::fmt::format(format_args!("*mut {0}", T::name()));
                res
            }
        }
    }
    impl<T: ?Sized + Reflected> ReflectedPointer for *mut T {
        const MUTABILITY: bool = true;
        fn element() -> Type {
            Type::of::<T>()
        }
    }
    unsafe impl<'a, T> NotOutlives<'a> for *mut T
    where
        T: ?Sized + NotOutlives<'a>,
    {}
    unsafe impl<T: ?Sized + Reflected> Reflected for &T {
        const TYPE: Type = Type::new_ref::<&T>();
        type Key = &'static T::Key;
        fn name() -> String {
            {
                let res = ::alloc::fmt::format(format_args!("&{0}", T::name()));
                res
            }
        }
        fn take_ref<'a, 'b>(val: &'a Value<'b>) -> Result<Value<'a>, Error>
        where
            Self: 'a + NotOutlives<'b>,
        {
            let new_ref = *unsafe { val.raw_ptr().cast::<&T>().as_ref() };
            let val = Value::from(new_ref);
            Ok(val)
        }
        fn take_mut<'a, 'b>(_: RefHack<'a, 'b>) -> Result<Value<'a>, Error>
        where
            Self: 'a + NotOutlives<'b>,
        {
            Err(Error::CantReborrow)
        }
    }
    impl<T: ?Sized + Reflected> ReflectedReference for &T {
        const MUTABILITY: bool = false;
        fn element() -> Type {
            Type::of::<T>()
        }
    }
    unsafe impl<'a, 'b, T: ?Sized> NotOutlives<'b> for &'b T
    where
        'b: 'a,
        T: NotOutlives<'a>,
    {}
    unsafe impl<T: ?Sized + Reflected> Reflected for &mut T {
        const TYPE: Type = Type::new_ref::<&mut T>();
        type Key = &'static mut T::Key;
        fn name() -> String {
            {
                let res = ::alloc::fmt::format(format_args!("&mut {0}", T::name()));
                res
            }
        }
        fn take_ref<'a, 'b>(_: &'a Value<'b>) -> Result<Value<'a>, Error>
        where
            Self: 'a + NotOutlives<'b>,
        {
            Err(Error::CantReborrow)
        }
        fn take_mut<'a, 'b>(_: RefHack<'a, 'b>) -> Result<Value<'a>, Error>
        where
            Self: 'a + NotOutlives<'b>,
        {
            Err(Error::CantReborrow)
        }
    }
    impl<T: ?Sized + Reflected> ReflectedReference for &mut T {
        const MUTABILITY: bool = true;
        fn element() -> Type {
            Type::of::<T>()
        }
    }
    unsafe impl<'a, 'b, T: ?Sized> NotOutlives<'b> for &'b mut T
    where
        'b: 'a,
        T: NotOutlives<'a>,
    {}
    unsafe impl<T: Reflected> Reflected for fn() -> T
    where
        T::Key: Sized,
    {
        const TYPE: Type = Type::new_fn::<fn() -> T>();
        type Key = fn() -> T::Key;
        fn name() -> String {
            {
                let res = ::alloc::fmt::format(format_args!("fn() -> {0}", T::name()));
                res
            }
        }
    }
    impl<T: Reflected> ReflectedFunction for fn() -> T
    where
        T::Key: Sized,
    {
        fn args() -> &'static [Type] {
            &[]
        }
        fn ret() -> Type {
            Type::of::<T>()
        }
    }
    unsafe impl<'no, T> NotOutlives<'no> for fn() -> T
    where
        T: NotOutlives<'no>,
    {}
    unsafe impl<T: Reflected, A0: Reflected> Reflected for fn(A0) -> T
    where
        A0::Key: Sized,
        T::Key: Sized,
    {
        const TYPE: Type = Type::new_fn::<fn(A0) -> T>();
        type Key = fn(A0::Key) -> T::Key;
        fn name() -> String {
            {
                let res = ::alloc::fmt::format(
                    format_args!("fn({0}) -> {1}", A0::name(), T::name()),
                );
                res
            }
        }
    }
    impl<T: Reflected, A0: Reflected> ReflectedFunction for fn(A0) -> T
    where
        A0::Key: Sized,
        T::Key: Sized,
    {
        fn args() -> &'static [Type] {
            &[A0::TYPE]
        }
        fn ret() -> Type {
            Type::of::<T>()
        }
    }
    unsafe impl<'no, A0, T> NotOutlives<'no> for fn(A0) -> T
    where
        T: NotOutlives<'no>,
        A0: NotOutlives<'no>,
    {}
    unsafe impl<T: Reflected, A0: Reflected, A1: Reflected> Reflected for fn(A0, A1) -> T
    where
        A0::Key: Sized,
        A1::Key: Sized,
        T::Key: Sized,
    {
        const TYPE: Type = Type::new_fn::<fn(A0, A1) -> T>();
        type Key = fn(A0::Key, A1::Key) -> T::Key;
        fn name() -> String {
            {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "fn({0}, {1}) -> {2}", A0::name(), A1::name(), T::name()
                    ),
                );
                res
            }
        }
    }
    impl<T: Reflected, A0: Reflected, A1: Reflected> ReflectedFunction
    for fn(A0, A1) -> T
    where
        A0::Key: Sized,
        A1::Key: Sized,
        T::Key: Sized,
    {
        fn args() -> &'static [Type] {
            &[A0::TYPE, A1::TYPE]
        }
        fn ret() -> Type {
            Type::of::<T>()
        }
    }
    unsafe impl<'no, A0, A1, T> NotOutlives<'no> for fn(A0, A1) -> T
    where
        T: NotOutlives<'no>,
        A0: NotOutlives<'no>,
        A1: NotOutlives<'no>,
    {}
    unsafe impl<T: Reflected, A0: Reflected, A1: Reflected, A2: Reflected> Reflected
    for fn(A0, A1, A2) -> T
    where
        A0::Key: Sized,
        A1::Key: Sized,
        A2::Key: Sized,
        T::Key: Sized,
    {
        const TYPE: Type = Type::new_fn::<fn(A0, A1, A2) -> T>();
        type Key = fn(A0::Key, A1::Key, A2::Key) -> T::Key;
        fn name() -> String {
            {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "fn({0}, {1}, {2}) -> {3}", A0::name(), A1::name(), A2::name(),
                        T::name()
                    ),
                );
                res
            }
        }
    }
    impl<T: Reflected, A0: Reflected, A1: Reflected, A2: Reflected> ReflectedFunction
    for fn(A0, A1, A2) -> T
    where
        A0::Key: Sized,
        A1::Key: Sized,
        A2::Key: Sized,
        T::Key: Sized,
    {
        fn args() -> &'static [Type] {
            &[A0::TYPE, A1::TYPE, A2::TYPE]
        }
        fn ret() -> Type {
            Type::of::<T>()
        }
    }
    unsafe impl<'no, A0, A1, A2, T> NotOutlives<'no> for fn(A0, A1, A2) -> T
    where
        T: NotOutlives<'no>,
        A0: NotOutlives<'no>,
        A1: NotOutlives<'no>,
        A2: NotOutlives<'no>,
    {}
}
