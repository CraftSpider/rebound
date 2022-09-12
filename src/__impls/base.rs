use crate::reflect::{
    Reflected, ReflectedArray, ReflectedFunction, ReflectedImpl, ReflectedPointer,
    ReflectedReference, ReflectedSlice, ReflectedTuple,
};
use crate::utils::StaticTypeMap;
use crate::value::NotOutlives;
use crate::{AssocConst, AssocFn, Error, Field, Type, Value};

use impl_trait_for_tuples::impl_for_tuples;
use rebound_proc::{extern_assoc_consts, extern_assoc_fns};

macro_rules! reflect_prims {
    ($($ty:ty),+ $(,)?) => {
        $(
        unsafe impl Reflected for $ty {
            type Key = $ty;

            fn ty() -> Type {
                Type::new_prim::<$ty>()
            }

            fn name() -> String {
                stringify!($ty).into()
            }
        }

        unsafe impl<'a> NotOutlives<'a> for $ty {}
        )*
    };
}

// Integers
reflect_prims! {
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

    bool,
    char,
    str,
}

impl ReflectedImpl<0> for u8 {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!(u8 @
            #[cfg(feature = "core")]
            fn from_str_radix(src: &str, radix: u32) -> Result<u8, core::num::ParseIntError>;
            const fn count_ones(self) -> u32;
            const fn count_zeros(self) -> u32;
            const fn leading_zeros(self) -> u32;
            const fn trailing_zeros(self) -> u32;
            const fn leading_ones(self) -> u32;
            const fn trailing_ones(self) -> u32;
            const fn rotate_left(self, n: u32) -> u8;
            const fn rotate_right(self, n: u32) -> u8;
            const fn swap_bytes(self) -> u8;
            const fn reverse_bits(self) -> u8;
            const fn from_be(x: u8) -> u8;
            const fn from_le(x: u8) -> u8;
            const fn to_be(self) -> u8;
            const fn to_le(self) -> u8;
            #[cfg(feature = "core")]
            const fn checked_add(self, rhs: u8) -> Option<u8>;
            // unsafe fn unchecked_add(self, rhs: u8) -> u8;
            #[cfg(feature = "core")]
            const fn checked_sub(self, rhs: u8) -> Option<u8>;
            // unsafe fn unchecked_sub(self, rhs: u8) -> u8;
            #[cfg(feature = "core")]
            const fn checked_mul(self, rhs: u8) -> Option<u8>;
            // unsafe fn unchecked_mul(self, rhs: u8) -> u8;
            #[cfg(feature = "core")]
            fn checked_div(self, rhs: u8) -> Option<u8>;
            #[cfg(feature = "core")]
            fn checked_div_euclid(self, rhs: u8) -> Option<u8>;
            #[cfg(feature = "core")]
            fn checked_rem(self, rhs: u8) -> Option<u8>;
            #[cfg(feature = "core")]
            fn checked_rem_euclid(self, rhs: u8) -> Option<u8>;
            #[cfg(feature = "core")]
            const fn checked_neg(self) -> Option<u8>;
            #[cfg(feature = "core")]
            const fn checked_shl(self, rhs: u32) -> Option<u8>;
            #[cfg(feature = "core")]
            const fn checked_shr(self, rhs: u32) -> Option<u8>;
            #[cfg(feature = "core")]
            fn checked_pow(self, exp: u32) -> Option<u8>;
            const fn saturating_add(self, rhs: u8) -> u8;
            const fn saturating_sub(self, rhs: u8) -> u8;
            const fn saturating_mul(self, rhs: u8) -> u8;
            fn saturating_pow(self, exp: u32) -> u8;
            const fn wrapping_add(self, rhs: u8) -> u8;
            const fn wrapping_sub(self, rhs: u8) -> u8;
            const fn wrapping_mul(self, rhs: u8) -> u8;
            fn wrapping_div(self, rhs: u8) -> u8;
            fn wrapping_div_euclid(self, rhs: u8) -> u8;
            fn wrapping_rem(self, rhs: u8) -> u8;
            fn wrapping_rem_euclid(self, rhs: u8) -> u8;
            const fn wrapping_neg(self) -> u8;
            const fn wrapping_shl(self, rhs: u32) -> u8;
            const fn wrapping_shr(self, rhs: u32) -> u8;
            fn wrapping_pow(self, exp: u32) -> u8;
            const fn overflowing_add(self, rhs: u8) -> (u8, bool);
            const fn overflowing_sub(self, rhs: u8) -> (u8, bool);
            const fn overflowing_mul(self, rhs: u8) -> (u8, bool);
            fn overflowing_div(self, rhs: u8) -> (u8, bool);
            fn overflowing_div_euclid(self, rhs: u8) -> (u8, bool);
            fn overflowing_rem(self, rhs: u8) -> (u8, bool);
            fn overflowing_rem_euclid(self, rhs: u8) -> (u8, bool);
            const fn overflowing_neg(self) -> (u8, bool);
            const fn overflowing_shl(self, rhs: u32) -> (u8, bool);
            const fn overflowing_shr(self, rhs: u32) -> (u8, bool);
            fn overflowing_pow(self, exp: u32) -> (u8, bool);
            fn pow(self, exp: u32) -> u8;
            fn div_euclid(self, rhs: u8) -> u8;
            fn rem_euclid(self, rhs: u8) -> u8;
            const fn is_power_of_two(self) -> bool;
            fn next_power_of_two(self) -> u8;
            #[cfg(feature = "core")]
            fn checked_next_power_of_two(self) -> Option<u8>;
            // fn wrapping_next_power_of_two(self) -> u8;
            const fn to_be_bytes(self) -> [u8; 1];
            const fn to_le_bytes(self) -> [u8; 1];
            const fn to_ne_bytes(self) -> [u8; 1];
            const fn from_be_bytes(bytes: [u8; 1]) -> u8;
            const fn from_le_bytes(bytes: [u8; 1]) -> u8;
            const fn from_ne_bytes(bytes: [u8; 1]) -> u8;
            // const fn min_value() -> u8;
            // const fn max_value() -> u8;
            const fn is_ascii(&self) -> bool;
            fn to_ascii_uppercase(&self) -> u8;
            fn to_ascii_lowercase(&self) -> u8;
            fn eq_ignore_ascii_case(&self, other: &u8) -> bool;
            fn make_ascii_uppercase(&mut self);
            fn make_ascii_lowercase(&mut self);
            const fn is_ascii_alphabetic(&self) -> bool;
            const fn is_ascii_uppercase(&self) -> bool;
            const fn is_ascii_lowercase(&self) -> bool;
            const fn is_ascii_alphanumeric(&self) -> bool;
            const fn is_ascii_digit(&self) -> bool;
            const fn is_ascii_hexdigit(&self) -> bool;
            const fn is_ascii_punctuation(&self) -> bool;
            const fn is_ascii_graphic(&self) -> bool;
            const fn is_ascii_whitespace(&self) -> bool;
            const fn is_ascii_control(&self) -> bool;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        extern_assoc_consts!(u8 @
            MIN: u8;
            MAX: u8;
            // BITS: u32;
        )
    }
}

// Floats

// Other raw types

impl ReflectedImpl<0> for bool {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!(bool @
            // fn then_some<T>(self, t: T) -> Option<T>;
            // fn then<T, F>(self, f: F) -> Option<T>;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl ReflectedImpl<0> for char {
    fn assoc_fns() -> Vec<AssocFn> {
        #[cfg(feature = "core")]
        use core::char::{EscapeDebug, EscapeDefault, EscapeUnicode, ToLowercase, ToUppercase};

        extern_assoc_fns!(char @
            // fn from_u32(i: u32) -> Option<char>;
            // unsafe fn from_u32_unchecked(i: u32) -> char;
            // fn from_digit(num: u32, radix: u32) -> Option<char>;
            fn is_digit(self, radix: u32) -> bool;
            #[cfg(feature = "core")]
            fn to_digit(self, radix: u32) -> Option<u32>;
            #[cfg(feature = "core")]
            fn escape_unicode(self) -> EscapeUnicode;
            #[cfg(feature = "core")]
            fn escape_debug(self) -> EscapeDebug;
            #[cfg(feature = "core")]
            fn escape_default(self) -> EscapeDefault;
            fn len_utf8(self) -> usize;
            fn len_utf16(self) -> usize;
            fn encode_utf8(self, dst: &mut [u8]) -> &mut str;
            fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16];
            fn is_alphabetic(self) -> bool;
            fn is_lowercase(self) -> bool;
            fn is_uppercase(self) -> bool;
            fn is_whitespace(self) -> bool;
            fn is_alphanumeric(self) -> bool;
            fn is_control(self) -> bool;
            fn is_numeric(self) -> bool;
            #[cfg(feature = "core")]
            fn to_lowercase(self) -> ToLowercase;
            #[cfg(feature = "core")]
            fn to_uppercase(self) -> ToUppercase;
            const fn is_ascii(&self) -> bool;
            fn to_ascii_uppercase(&self) -> char;
            fn to_ascii_lowercase(&self) -> char;
            fn eq_ignore_ascii_case(&self, other: &char) -> bool;
            fn make_ascii_uppercase(&mut self);
            fn make_ascii_lowercase(&mut self);
            const fn is_ascii_alphabetic(&self) -> bool;
            const fn is_ascii_uppercase(&self) -> bool;
            const fn is_ascii_lowercase(&self) -> bool;
            const fn is_ascii_alphanumeric(&self) -> bool;
            const fn is_ascii_digit(&self) -> bool;
            const fn is_ascii_hexdigit(&self) -> bool;
            const fn is_ascii_punctuation(&self) -> bool;
            const fn is_ascii_graphic(&self) -> bool;
            const fn is_ascii_whitespace(&self) -> bool;
            const fn is_ascii_control(&self) -> bool;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        extern_assoc_consts!(char @
            // MAX: char;
            // REPLACEMENT_CHARACTER: char;
            // UNICODE_VERSION: (u8, u8, u8);
        )
    }
}

impl ReflectedImpl<0> for str {
    fn assoc_fns() -> Vec<AssocFn> {
        #[cfg(feature = "core")]
        use core::str::{
            Bytes, CharIndices, Chars, EncodeUtf16, EscapeDebug, EscapeDefault, EscapeUnicode,
            Lines, SplitAsciiWhitespace, SplitWhitespace,
        };

        extern_assoc_fns!(str @
            const fn len(&self) -> usize;
            const fn is_empty(&self) -> bool;
            fn is_char_boundary(&self, index: usize) -> bool;
            const fn as_bytes(&self) -> &[u8];
            unsafe fn as_bytes_mut(&mut self) -> &mut [u8];
            const fn as_ptr(&self) -> *const u8;
            fn as_mut_ptr(&mut self) -> *mut u8;
            fn split_at(&self, mid: usize) -> (&str, &str);
            fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str);
            #[cfg(feature = "core")]
            fn chars(&self) -> Chars<'_>;
            #[cfg(feature = "core")]
            fn char_indices(&self) -> CharIndices<'_>;
            #[cfg(feature = "core")]
            fn bytes(&self) -> Bytes<'_>;
            #[cfg(feature = "core")]
            fn split_whitespace(&self) -> SplitWhitespace<'_>;
            #[cfg(feature = "core")]
            fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_>;
            #[cfg(feature = "core")]
            fn lines(&self) -> Lines<'_>;
            #[cfg(feature = "core")]
            fn encode_utf16(&self) -> EncodeUtf16<'_>;
            fn trim(&self) -> &str;
            fn trim_start(&self) -> &str;
            fn trim_end(&self) -> &str;
            fn is_ascii(&self) -> bool;
            fn eq_ignore_ascii_case(&self, other: &str) -> bool;
            fn make_ascii_uppercase(&mut self);
            fn make_ascii_lowercase(&mut self);
            #[cfg(feature = "core")]
            fn escape_debug(&self) -> EscapeDebug<'_>;
            #[cfg(feature = "core")]
            fn escape_default(&self) -> EscapeDefault<'_>;
            #[cfg(feature = "core")]
            fn escape_unicode(&self) -> EscapeUnicode<'_>;
            #[cfg(feature = "alloc")]
            fn into_boxed_bytes(self: Box<str>) -> Box<[u8]>;
            #[cfg(feature = "alloc")]
            fn to_lowercase(&self) -> String;
            #[cfg(feature = "alloc")]
            fn to_uppercase(&self) -> String;
            #[cfg(feature = "alloc")]
            fn into_string(self: Box<str>) -> String;
            #[cfg(feature = "alloc")]
            fn repeat(&self, n: usize) -> String;
            #[cfg(feature = "alloc")]
            fn to_ascii_uppercase(&self) -> String;
            #[cfg(feature = "alloc")]
            fn to_ascii_lowercase(&self) -> String;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

// Tuple reflections

unsafe impl Reflected for () {
    type Key = ();

    fn ty() -> Type {
        Type::new_tuple::<()>()
    }

    fn name() -> String {
        "()".into()
    }
}

impl ReflectedTuple for () {
    fn fields() -> &'static [Field] {
        &[]
    }
}

unsafe impl<'a> NotOutlives<'a> for () {}

#[impl_for_tuples(1, 26)]
unsafe impl Reflected for Tuple {
    for_tuples!( type Key = ( #( Tuple::Key ),* ); );
    for_tuples!( where #(Tuple::Key: Sized)* );

    fn ty() -> Type {
        Type::new_tuple::<Self>()
    }

    fn name() -> String {
        let names = [for_tuples!( #(Tuple::name()),* )];
        format!("({})", names.join(", "))
    }
}

#[impl_for_tuples(1, 26)]
#[tuple_types_custom_trait_bound(Reflected)]
impl ReflectedTuple for Tuple {
    for_tuples!( where #( Tuple::Key: Sized )* );

    fn fields() -> &'static [Field] {
        static TUPLE_FIELDS: StaticTypeMap<Vec<Field>> = StaticTypeMap::new();

        TUPLE_FIELDS.call_once::<Self, fn() -> _>(|| {
            use crate::info::{AccessHelper, SetHelper};
            use crate::value::Value;

            let mut idx_count = 0;

            // HACK: idx_count used because the macro provides no easy way to get the current
            //       index.
            #[allow(clippy::mixed_read_write_in_expression)]
            Vec::from([for_tuples!( #( {
                let get_ptr: Option<AccessHelper> = Some(|this| {
                    // SAFETY: We know we won't borrow the item past the lifetime of the
                    //         containing value
                    let inner = unsafe { this.borrow_unsafe::<Self>() };
                    let v = Value::from_ref(&inner.Tuple);
                    // SAFETY: See rebound::ty::Ref
                    unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                });

                let set_ptr: Option<SetHelper> = Some(|this, value| {
                    // SAFETY: We know we won't borrow the item past the lifetimes off the
                    //         containing value
                    let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                    // SAFETY: The passed value is expected to be static, so we can only
                    //         cast the lifetime lower here
                    inner.Tuple = unsafe { value.cast_unsafe::<Tuple>() };
                });

                let idx = idx_count;
                idx_count += 1;

                let assoc_ty = Type::of::<Self>();
                let field_ty = Type::of::<Tuple>();

                // SAFETY: We're the privileged implementation
                unsafe { Field::new_tuple(get_ptr, set_ptr, idx, assoc_ty, field_ty) }

            } ),* )])
        })
    }
}

macro_rules! tuple_no {
    ($first:ident $first_lt:lifetime $($remaining:ident $remaining_lt:lifetime)*) => {
        unsafe impl<'no, $first_lt, $($remaining_lt,)* $first, $($remaining,)*> NotOutlives<'no> for ($first, $($remaining),*)
        where
            'no: $first_lt $(+ $remaining_lt)*,
            $first: NotOutlives<$first_lt>,
            $(
            $remaining: NotOutlives<$remaining_lt>,
            )*
        {}

        tuple_no!($($remaining $remaining_lt)*);
    };
    () => {};
}

tuple_no!(
    A 'a B 'b C 'c D 'd E 'e F 'f G 'g H 'h I 'i J 'j K 'k L 'l M 'm N 'n O 'o P 'p Q 'q R 'r S 's
    T 't U 'u V 'v W 'w X 'x Y 'y Z 'z
);

// Arrays/Slices
unsafe impl<T, const N: usize> Reflected for [T; N]
where
    T: Reflected,
    T::Key: Sized,
{
    type Key = [T::Key; N];

    fn ty() -> Type {
        Type::new_array::<[T; N]>()
    }

    fn name() -> String {
        format!("[{}; {}]", T::name(), N)
    }
}

impl<T, const N: usize> ReflectedArray for [T; N]
where
    T: Reflected,
    T::Key: Sized,
{
    fn element() -> Type {
        Type::of::<T>()
    }

    fn length() -> usize {
        N
    }
}

unsafe impl<'a, T, const N: usize> NotOutlives<'a> for [T; N] where T: NotOutlives<'a> {}

unsafe impl<T> Reflected for [T]
where
    T: Reflected,
    T::Key: Sized,
{
    type Key = [T::Key];

    fn ty() -> Type {
        Type::new_slice::<[T]>()
    }

    fn name() -> String {
        format!("[{}]", T::name())
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
{
}

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
            Chunks, ChunksExact, ChunksExactMut, ChunksMut, Iter, IterMut, RChunks, RChunksExact,
            RChunksExactMut, RChunksMut, Windows,
        };

        extern_assoc_fns!([T] @
            const fn len(&self) -> usize;
            const fn is_empty(&self) -> bool;
            #[cfg(feature = "core")]
            fn first(&self) -> Option<&T>;
            #[cfg(feature = "core")]
            fn first_mut(&mut self) -> Option<&mut T>;
            #[cfg(feature = "core")]
            fn split_first(&self) -> Option<(&T, &[T])>;
            #[cfg(feature = "core")]
            fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>;
            #[cfg(feature = "core")]
            fn split_last(&self) -> Option<(&T, &[T])>;
            #[cfg(feature = "core")]
            fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])>;
            #[cfg(feature = "core")]
            fn last(&self) -> Option<&T>;
            #[cfg(feature = "core")]
            fn last_mut(&mut self) -> Option<&mut T>;
            const fn as_ptr(&self) -> *const T;
            fn as_mut_ptr(&mut self) -> *mut T;
            #[cfg(feature = "core")]
            fn as_ptr_range(&self) -> Range<*const T>;
            #[cfg(feature = "core")]
            fn as_mut_ptr_range(&mut self) -> Range<*mut T>;
            fn swap(&mut self, a: usize, b: usize);
            fn reverse(&mut self);
            #[cfg(feature = "core")]
            fn iter(&self) -> Iter<'_, T>;
            #[cfg(feature = "core")]
            fn iter_mut(&mut self) -> IterMut<'_, T>;
            #[cfg(feature = "core")]
            fn windows(&self, size: usize) -> Windows<'_, T>;
            #[cfg(feature = "core")]
            fn chunks(&self, chunk_size: usize) -> Chunks<'_, T>;
            #[cfg(feature = "core")]
            fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T>;
            #[cfg(feature = "core")]
            fn chunks_exact(&self, chunk_size: usize) -> ChunksExact<'_, T>;
            #[cfg(feature = "core")]
            fn chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<'_, T>;
            #[cfg(feature = "core")]
            fn rchunks(&self, chunk_size: usize) -> RChunks<'_, T>;
            #[cfg(feature = "core")]
            fn rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<'_, T>;
            #[cfg(feature = "core")]
            fn rchunks_exact(&self, chunk_size: usize) -> RChunksExact<'_, T>;
            #[cfg(feature = "core")]
            fn rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<'_, T>;
            fn split_at(&self, mid: usize) -> (&[T], &[T]);
            fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]);
            fn rotate_left(&mut self, mid: usize);
            fn rotate_right(&mut self, k: usize);
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T> ReflectedImpl<1> for [T]
where
    T: Reflected + PartialEq,
    T::Key: Sized,
{
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
            fn contains(&self, x: &T) -> bool;
            fn starts_with(&self, needle: &[T]) -> bool;
            fn ends_with(&self, needle: &[T]) -> bool;
            // fn strip_prefix(&self, prefix: &[T]) -> Option<&[T]>;
            // fn strip_suffix(&self, suffix: &[T]) -> Option<&[T]>;
            // fn partition_dedup(&mut self) -> (&mut [T], &mut [T]);
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T> ReflectedImpl<2> for [T]
where
    T: Reflected + PartialOrd,
    T::Key: Sized,
{
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
            // fn is_sorted(&self) -> bool;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T> ReflectedImpl<3> for [T]
where
    T: Reflected + Ord,
    T::Key: Sized,
{
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
            fn sort(&mut self);
            fn sort_unstable(&mut self);
            #[cfg(feature = "core")]
            fn binary_search(&self, x: &T) -> Result<usize, usize>;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T> ReflectedImpl<4> for [T]
where
    T: Reflected + Clone,
    T::Key: Sized,
{
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
            // fn fill(&mut self, value: T);
            fn clone_from_slice(&mut self, src: &[T]);
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T> ReflectedImpl<5> for [T]
where
    T: Reflected + Clone + 'static,
    T::Key: Reflected + Sized,
    <T::Key as Reflected>::Key: Reflected + Sized,
{
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
            #[cfg(feature = "alloc")]
            fn to_vec(&self) -> Vec<T>;
            #[cfg(feature = "alloc")]
            fn into_vec(self: Box<[T]>) -> Vec<T>;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T> ReflectedImpl<6> for [T]
where
    T: Reflected + Copy,
    T::Key: Sized,
{
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
            fn copy_from_slice(&mut self, src: &[T]);
            fn swap_with_slice(&mut self, other: &mut [T]);
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T> ReflectedImpl<7> for [T]
where
    T: Reflected + Copy + 'static,
    T::Key: Reflected + Sized,
{
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
            #[cfg(feature = "alloc")]
            fn repeat(&self, n: usize) -> Vec<T>;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl ReflectedImpl<8> for [u8] {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([u8] @
            fn is_ascii(&self) -> bool;
            fn eq_ignore_ascii_case(&self, other: &[u8]) -> bool;
            fn make_ascii_uppercase(&mut self);
            fn make_ascii_lowercase(&mut self);
            #[cfg(feature = "alloc")]
            fn to_ascii_uppercase(&self) -> Vec<u8>;
            #[cfg(feature = "alloc")]
            fn to_ascii_lowercase(&self) -> Vec<u8>;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

// Pointers
unsafe impl<T: ?Sized + Reflected> Reflected for *const T {
    type Key = *const T::Key;

    fn ty() -> Type {
        Type::new_ptr::<*const T>()
    }

    fn name() -> String {
        format!("*const {}", T::name())
    }
}

impl<T: ?Sized + Reflected> ReflectedPointer for *const T {
    fn element() -> Type {
        Type::of::<T>()
    }

    fn mutability() -> bool {
        false
    }
}

unsafe impl<'a, T> NotOutlives<'a> for *const T where T: NotOutlives<'a> {}

impl<T: ?Sized + Reflected> ReflectedImpl<0> for *const T {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!(*const T @
            fn is_null(self) -> bool;
            // fn guaranteed_eq(self, other: *const T) -> bool;
            // fn guaranteed_ne(self, other: *const T) -> bool;
            // fn set_ptr_value(self, val: *const u8) -> *const T;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T: Reflected> ReflectedImpl<1> for *const T {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!(*const T @
            unsafe fn offset_from(self, origin: *const T) -> isize;
            unsafe fn copy_to(self, dest: *mut T, count: usize);
            unsafe fn copy_to_nonoverlapping(self, dest: *mut T, count: usize);
            fn align_offset(self, align: usize) -> usize;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

// Needed because AssocFn requires static lived output. This may be fixable?
impl<T: Reflected + 'static> ReflectedImpl<2> for *const T {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!(*const T @
            unsafe fn offset(self, count: isize) -> *const T;
            fn wrapping_offset(self, count: isize) -> *const T;
            unsafe fn read(self) -> T;
            unsafe fn read_volatile(self) -> T;
            unsafe fn read_unaligned(self) -> T;
            unsafe fn add(self, count: usize) -> *const T;
            unsafe fn sub(self, count: usize) -> *const T;
            fn wrapping_add(self, count: usize) -> *const T;
            fn wrapping_sub(self, count: usize) -> *const T;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

unsafe impl<T: ?Sized + Reflected> Reflected for *mut T {
    type Key = *mut T::Key;

    fn ty() -> Type {
        Type::new_ptr::<*mut T>()
    }

    fn name() -> String {
        format!("*mut {}", T::name())
    }
}

impl<T: ?Sized + Reflected> ReflectedPointer for *mut T {
    fn element() -> Type {
        Type::of::<T>()
    }

    fn mutability() -> bool {
        true
    }
}

unsafe impl<'a, T> NotOutlives<'a> for *mut T where T: NotOutlives<'a> {}

// References
unsafe impl<T: ?Sized + Reflected> Reflected for &T {
    type Key = &'static T::Key;

    fn ty() -> Type {
        Type::new_ref::<&T>()
    }

    fn name() -> String {
        format!("&{}", T::name())
    }

    fn take_ref<'a>(val: &'a Value<'_>) -> Result<Value<'a>, Error> {
        let new_ref = *unsafe { val.raw_ptr().cast::<&T>().as_ref() };
        let val = Value::from(new_ref);
        // SAFETY: See comment on default impl
        Ok(unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(val) })
    }

    fn take_mut<'a>(_: &'a mut Value<'_>) -> Result<Value<'a>, Error> {
        Err(Error::CantReborrow)
    }
}

impl<T: ?Sized + Reflected> ReflectedReference for &T {
    fn element() -> Type {
        Type::of::<T>()
    }

    fn mutability() -> bool {
        false
    }
}

unsafe impl<'a, 'b, T: ?Sized> NotOutlives<'b> for &'b T
where
    'b: 'a,
    T: NotOutlives<'a>,
{
}

unsafe impl<T: ?Sized + Reflected> Reflected for &mut T {
    type Key = &'static mut T::Key;

    fn ty() -> Type {
        Type::new_ref::<&mut T>()
    }

    fn name() -> String {
        format!("&mut {}", T::name())
    }

    fn take_ref<'a>(_: &'a Value<'_>) -> Result<Value<'a>, Error> {
        Err(Error::CantReborrow)
    }

    fn take_mut<'a>(_: &'a mut Value<'_>) -> Result<Value<'a>, Error> {
        Err(Error::CantReborrow)
    }
}

impl<T: ?Sized + Reflected> ReflectedReference for &mut T {
    fn element() -> Type {
        Type::of::<T>()
    }

    fn mutability() -> bool {
        true
    }
}

unsafe impl<'a, 'b, T: ?Sized> NotOutlives<'b> for &'b mut T
where
    'b: 'a,
    T: NotOutlives<'a>,
{
}

// Function pointers
unsafe impl<T: Reflected> Reflected for fn() -> T {
    type Key = fn() -> T::Key;

    fn ty() -> Type {
        Type::new_fn::<fn() -> T>()
    }

    fn name() -> String {
        format!("fn() -> {}", T::name())
    }
}

impl<T: Reflected> ReflectedFunction for fn() -> T {
    fn args() -> Vec<Type> {
        vec![]
    }

    fn ret() -> Type {
        Type::of::<T>()
    }
}

unsafe impl<T: Reflected, A0: Reflected> Reflected for fn(A0) -> T {
    type Key = fn(A0::Key) -> T::Key;

    fn ty() -> Type {
        Type::new_fn::<fn(A0) -> T>()
    }

    fn name() -> String {
        format!("fn({}) -> {}", A0::name(), T::name())
    }
}

impl<T: Reflected, A0: Reflected> ReflectedFunction for fn(A0) -> T {
    fn args() -> Vec<Type> {
        vec![Type::of::<A0>()]
    }

    fn ret() -> Type {
        Type::of::<T>()
    }
}

unsafe impl<T: Reflected, A0: Reflected, A1: Reflected> Reflected for fn(A0, A1) -> T {
    type Key = fn(A0::Key, A1::Key) -> T::Key;

    fn ty() -> Type {
        Type::new_fn::<fn(A0, A1) -> T>()
    }

    fn name() -> String {
        format!("fn({}, {}) -> {}", A0::name(), A1::name(), T::name())
    }
}

impl<T: Reflected, A0: Reflected, A1: Reflected> ReflectedFunction for fn(A0, A1) -> T {
    fn args() -> Vec<Type> {
        vec![Type::of::<A0>(), Type::of::<A1>()]
    }

    fn ret() -> Type {
        Type::of::<T>()
    }
}

unsafe impl<T: Reflected, A0: Reflected, A1: Reflected, A2: Reflected> Reflected
    for fn(A0, A1, A2) -> T
{
    type Key = fn(A0::Key, A1::Key, A2::Key) -> T::Key;

    fn ty() -> Type {
        Type::new_fn::<fn(A0, A1, A2) -> T>()
    }

    fn name() -> String {
        format!(
            "fn({}, {}, {}) -> {}",
            A0::name(),
            A1::name(),
            A2::name(),
            T::name()
        )
    }
}

impl<T: Reflected, A0: Reflected, A1: Reflected, A2: Reflected> ReflectedFunction
    for fn(A0, A1, A2) -> T
{
    fn args() -> Vec<Type> {
        vec![Type::of::<A0>(), Type::of::<A1>(), Type::of::<A2>()]
    }

    fn ret() -> Type {
        Type::of::<T>()
    }
}

// Never type
#[cfg(feature = "never-type")]
unsafe impl Reflected for ! {
    type Key = !;

    fn ty() -> Type {
        Type::new_prim::<!>()
    }

    fn name() -> String {
        "!".into()
    }
}
