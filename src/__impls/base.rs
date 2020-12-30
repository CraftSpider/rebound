use crate::__helpers::*;
use crate::reflect::*;
use crate::{AssocConst, AssocFn, Field, Type};

use rebound_proc::{extern_assoc_consts, extern_assoc_fns, reflect_prims};

// TODO: Add impls for all these

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

    char,
    bool,
}

impl ReflectedImpl<0> for u8 {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!(u8 @
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
            const fn checked_add(self, rhs: u8) -> Option<u8>;
            // unsafe fn unchecked_add(self, rhs: u8) -> u8;
            const fn checked_sub(self, rhs: u8) -> Option<u8>;
            // unsafe fn unchecked_sub(self, rhs: u8) -> u8;
            const fn checked_mul(self, rhs: u8) -> Option<u8>;
            // unsafe fn unchecked_mul(self, rhs: u8) -> u8;
            fn checked_div(self, rhs: u8) -> Option<u8>;
            fn checked_div_euclid(self, rhs: u8) -> Option<u8>;
            fn checked_rem(self, rhs: u8) -> Option<u8>;
            fn checked_rem_euclid(self, rhs: u8) -> Option<u8>;
            const fn checked_neg(self) -> Option<u8>;
            const fn checked_shl(self, rhs: u32) -> Option<u8>;
            const fn checked_shr(self, rhs: u32) -> Option<u8>;
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
        use core::char::{EscapeDebug, EscapeDefault, EscapeUnicode, ToLowercase, ToUppercase};

        extern_assoc_fns!(char @
            // fn from_u32(i: u32) -> Option<char>;
            // unsafe fn from_u32_unchecked(i: u32) -> char;
            // fn from_digit(num: u32, radix: u32) -> Option<char>;
            fn is_digit(self, radix: u32) -> bool;
            fn to_digit(self, radix: u32) -> Option<u32>;
            fn escape_unicode(self) -> EscapeUnicode;
            fn escape_debug(self) -> EscapeDebug;
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
            fn to_lowercase(self) -> ToLowercase;
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

impl Reflected for str {
    type Meta = usize;

    fn name() -> String {
        "str".into()
    }

    fn assemble(meta: Self::Meta, ptr: *mut ()) -> *mut Self {
        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr as *const u8, meta))
                as *const str as _
        }
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        self.as_bytes().disassemble()
    }

    unsafe fn init() {
        Type::new_prim::<str>()
    }
}

impl ReflectedImpl<0> for str {
    fn assoc_fns() -> Vec<AssocFn> {
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
            fn chars(&self) -> Chars;
            fn char_indices(&self) -> CharIndices;
            fn bytes(&self) -> Bytes;
            fn split_whitespace(&self) -> SplitWhitespace;
            fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace;
            fn lines(&self) -> Lines;
            fn encode_utf16(&self) -> EncodeUtf16;
            fn trim(&self) -> &str;
            fn trim_start(&self) -> &str;
            fn trim_end(&self) -> &str;
            fn is_ascii(&self) -> bool;
            fn eq_ignore_ascii_case(&self, other: &str) -> bool;
            fn make_ascii_uppercase(&mut self);
            fn make_ascii_lowercase(&mut self);
            fn escape_debug(&self) -> EscapeDebug;
            fn escape_default(&self) -> EscapeDefault;
            fn escape_unicode(&self) -> EscapeUnicode;
            fn into_boxed_bytes(self: Box<str>) -> Box<[u8]>;
            fn to_lowercase(&self) -> String;
            fn to_uppercase(&self) -> String;
            fn into_string(self: Box<str>) -> String;
            fn repeat(&self, n: usize) -> String;
            fn to_ascii_uppercase(&self) -> String;
            fn to_ascii_lowercase(&self) -> String;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

// Tuple reflections
impl Reflected for () {
    fn name() -> String {
        "()".into()
    }

    fn assemble(_: Self::Meta, ptr: *mut ()) -> *mut Self {
        ptr as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        ((), self as *const Self as _)
    }

    unsafe fn init() {
        Type::new_tuple::<()>()
    }
}

impl ReflectedTuple for () {
    fn fields() -> Vec<Field> {
        vec![]
    }
}

// TODO: Make this valid for non-static lifetimes maybe
impl<T0> Reflected for (T0,)
where
    T0: Reflected + 'static,
{
    fn name() -> String {
        format!("({},)", T0::name())
    }

    fn assemble(_: Self::Meta, ptr: *mut ()) -> *mut Self {
        ptr as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        ((), self as *const Self as _)
    }

    unsafe fn init() {
        Type::new_tuple::<(T0,)>()
    }
}

impl<T0> ReflectedTuple for (T0,)
where
    T0: Reflected + 'static,
{
    fn fields() -> Vec<Field> {
        unsafe {
            vec![Field::new_tuple(
                Some(__make_ref_accessor!((T0,), 0)),
                Some(__make_setter!((T0,), 0)),
                0,
                Type::from::<(T0,)>(),
                Type::from::<T0>(),
            )]
        }
    }
}

impl<T0, T1> Reflected for (T0, T1)
where
    T0: Reflected + 'static,
    T1: Reflected + 'static,
{
    fn name() -> String {
        format!("({}, {})", T0::name(), T1::name())
    }

    fn assemble(_: Self::Meta, ptr: *mut ()) -> *mut Self {
        ptr as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        ((), self as *const Self as _)
    }

    unsafe fn init() {
        Type::new_tuple::<(T0, T1)>()
    }
}

impl<T0, T1> ReflectedTuple for (T0, T1)
where
    T0: Reflected + 'static,
    T1: Reflected + 'static,
{
    fn fields() -> Vec<Field> {
        unsafe {
            vec![
                Field::new_tuple(
                    Some(__make_ref_accessor!((T0, T1), 0)),
                    Some(__make_setter!((T0, T1), 0)),
                    0,
                    Type::from::<(T0, T1)>(),
                    Type::from::<T0>(),
                ),
                Field::new_tuple(
                    Some(__make_ref_accessor!((T0, T1), 1)),
                    Some(__make_setter!((T0, T1), 1)),
                    1,
                    Type::from::<(T0, T1)>(),
                    Type::from::<T1>(),
                ),
            ]
        }
    }
}

impl<T0, T1, T2> Reflected for (T0, T1, T2)
where
    T0: Reflected + 'static,
    T1: Reflected + 'static,
    T2: Reflected + 'static,
{
    fn name() -> String {
        format!("({}, {}, {})", T0::name(), T1::name(), T2::name())
    }

    fn assemble(_: Self::Meta, ptr: *mut ()) -> *mut Self {
        ptr as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        ((), self as *const Self as _)
    }

    unsafe fn init() {
        Type::new_tuple::<(T0, T1, T2)>()
    }
}

impl<T0, T1, T2> ReflectedTuple for (T0, T1, T2)
where
    T0: Reflected + 'static,
    T1: Reflected + 'static,
    T2: Reflected + 'static,
{
    fn fields() -> Vec<Field> {
        unsafe {
            vec![
                Field::new_tuple(
                    Some(__make_ref_accessor!((T0, T1, T2), 0)),
                    Some(__make_setter!((T0, T1, T2), 0)),
                    0,
                    Type::from::<(T0, T1, T2)>(),
                    Type::from::<T0>(),
                ),
                Field::new_tuple(
                    Some(__make_ref_accessor!((T0, T1, T2), 1)),
                    Some(__make_setter!((T0, T1, T2), 1)),
                    1,
                    Type::from::<(T0, T1, T2)>(),
                    Type::from::<T1>(),
                ),
                Field::new_tuple(
                    Some(__make_ref_accessor!((T0, T1, T2), 2)),
                    Some(__make_setter!((T0, T1, T2), 2)),
                    2,
                    Type::from::<(T0, T1, T2)>(),
                    Type::from::<T1>(),
                ),
            ]
        }
    }
}

// Arrays/Slices
impl<T: Reflected, const N: usize> Reflected for [T; N] {
    fn name() -> String {
        format!("[{}; {}]", T::name(), N)
    }

    fn assemble(_: Self::Meta, ptr: *mut ()) -> *mut Self {
        ptr as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        ((), self as *const Self as _)
    }

    unsafe fn init() {
        Type::new_array::<[T; N]>()
    }
}

impl<T: Reflected, const N: usize> ReflectedArray for [T; N] {
    fn element() -> Type {
        Type::from::<T>()
    }

    fn length() -> usize {
        N
    }
}

impl<T: Reflected> Reflected for [T] {
    type Meta = usize;

    fn name() -> String {
        format!("[{}]", T::name())
    }

    fn assemble(meta: Self::Meta, ptr: *mut ()) -> *mut Self {
        unsafe { core::slice::from_raw_parts_mut(ptr as *mut T, meta) as _ }
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        (self.len(), self.as_ptr() as _)
    }

    unsafe fn init() {
        Type::new_slice::<[T]>()
    }
}

impl<T: Reflected> ReflectedSlice for [T] {
    fn element() -> Type {
        Type::from::<T>()
    }
}

// TODO: Split into 'static and non-'static impls.
impl<T: Reflected + 'static> ReflectedImpl<0> for [T] {
    fn assoc_fns() -> Vec<AssocFn> {
        use core::ops::Range;
        use core::slice::{
            Chunks, ChunksExact, ChunksExactMut, ChunksMut, Iter, IterMut, RChunks, RChunksExact,
            RChunksExactMut, RChunksMut, Windows,
        };

        extern_assoc_fns!([T] @
            const fn len(&self) -> usize;
            const fn is_empty(&self) -> bool;
            fn first(&self) -> Option<&T>;
            fn first_mut(&mut self) -> Option<&mut T>;
            fn split_first(&self) -> Option<(&T, &[T])>;
            fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>;
            fn split_last(&self) -> Option<(&T, &[T])>;
            fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])>;
            fn last(&self) -> Option<&T>;
            fn last_mut(&mut self) -> Option<&mut T>;
            const fn as_ptr(&self) -> *const T;
            fn as_mut_ptr(&mut self) -> *mut T;
            fn as_ptr_range(&self) -> Range<*const T>;
            fn as_mut_ptr_range(&mut self) -> Range<*mut T>;
            fn swap(&mut self, a: usize, b: usize);
            fn reverse(&mut self);
            fn iter(&self) -> Iter<'_, T>;
            fn iter_mut(&mut self) -> IterMut<'_, T>;
            fn windows(&self, size: usize) -> Windows<'_, T>;
            fn chunks(&self, chunk_size: usize) -> Chunks<'_, T>;
            fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T>;
            fn chunks_exact(&self, chunk_size: usize) -> ChunksExact<'_, T>;
            fn chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<'_, T>;
            fn rchunks(&self, chunk_size: usize) -> RChunks<'_, T>;
            fn rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<'_, T>;
            fn rchunks_exact(&self, chunk_size: usize) -> RChunksExact<'_, T>;
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

impl<T: Reflected + PartialEq> ReflectedImpl<1> for [T] {
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

impl<T: Reflected + PartialOrd> ReflectedImpl<2> for [T] {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
            // fn is_sorted(&self) -> bool;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T: Reflected + Ord> ReflectedImpl<3> for [T] {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
            fn sort(&mut self);
            fn sort_unstable(&mut self);
            fn binary_search(&self, x: &T) -> Result<usize, usize>;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T: Reflected + Clone> ReflectedImpl<4> for [T] {
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

impl<T: Reflected + Clone + 'static> ReflectedImpl<5> for [T] {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
            fn to_vec(&self) -> Vec<T>;
            fn into_vec(self: Box<[T]>) -> Vec<T>;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

impl<T: Reflected + Copy> ReflectedImpl<6> for [T] {
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

impl<T: Reflected + Copy + 'static> ReflectedImpl<7> for [T] {
    fn assoc_fns() -> Vec<AssocFn> {
        extern_assoc_fns!([T] @
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
            fn to_ascii_uppercase(&self) -> Vec<u8>;
            fn to_ascii_lowercase(&self) -> Vec<u8>;
        )
    }

    fn assoc_consts() -> Vec<AssocConst> {
        vec![]
    }
}

// Pointers
impl<T: ?Sized + Reflected> Reflected for *const T {
    type Meta = T::Meta;

    fn name() -> String {
        format!("*const {}", T::name())
    }

    fn assemble(meta: Self::Meta, ptr: *mut ()) -> *mut Self {
        &mut T::assemble(meta, unsafe { *(ptr as *mut *mut ()) }) as *mut *mut T as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        (
            T::disassemble(unsafe { &**self }).0,
            self as *const *const T as _,
        )
    }

    unsafe fn init() {
        Type::new_ptr::<*const T>();
    }
}

impl<T: ?Sized + Reflected> ReflectedPointer for *const T {
    fn element() -> Type {
        Type::from::<T>()
    }

    fn mutability() -> bool {
        false
    }
}

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

impl<T: ?Sized + Reflected> Reflected for *mut T {
    type Meta = T::Meta;

    fn name() -> String {
        format!("*mut {}", T::name())
    }

    fn assemble(meta: Self::Meta, ptr: *mut ()) -> *mut Self {
        &mut T::assemble(meta, unsafe { *(ptr as *mut *mut ()) }) as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        (
            T::disassemble(unsafe { &mut **self }).0,
            self as *const *mut T as _,
        )
    }

    unsafe fn init() {
        Type::new_ptr::<*mut T>();
    }
}

impl<T: ?Sized + Reflected> ReflectedPointer for *mut T {
    fn element() -> Type {
        Type::from::<T>()
    }

    fn mutability() -> bool {
        true
    }
}

// References
impl<T: ?Sized + Reflected> Reflected for &T {
    type Meta = T::Meta;

    fn name() -> String {
        format!("&{}", T::name())
    }

    fn assemble(meta: Self::Meta, ptr: *mut ()) -> *mut Self {
        &mut T::assemble(meta, unsafe { *(ptr as *const *mut ()) }) as *mut *mut T as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        (T::disassemble(&**self).0, self as *const &T as _)
    }

    unsafe fn init() {
        Type::new_ref::<&T>();
    }
}

impl<T: ?Sized + Reflected> ReflectedReference for &T {
    fn element() -> Type {
        Type::from::<T>()
    }

    fn mutability() -> bool {
        false
    }
}

impl<T: ?Sized + Reflected> Reflected for &mut T {
    type Meta = T::Meta;

    fn name() -> String {
        format!("&mut {}", T::name())
    }

    fn assemble(meta: Self::Meta, ptr: *mut ()) -> *mut Self {
        &mut T::assemble(meta, unsafe { *(ptr as *mut *mut ()) }) as *mut *mut T as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        (T::disassemble(*self).0, self as *const &mut T as _)
    }

    unsafe fn init() {
        Type::new_ref::<&mut T>();
    }
}

impl<T: ?Sized + Reflected> ReflectedReference for &mut T {
    fn element() -> Type {
        Type::from::<T>()
    }

    fn mutability() -> bool {
        true
    }
}

// Function pointers
impl<T: Reflected> Reflected for fn() -> T {
    fn name() -> String {
        format!("fn() -> {}", T::name())
    }

    fn assemble(_: Self::Meta, ptr: *mut ()) -> *mut Self {
        ptr as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        ((), self as *const Self as _)
    }

    unsafe fn init() {
        Type::new_fn::<fn() -> T>()
    }
}

impl<T: Reflected> ReflectedFunction for fn() -> T {
    fn args() -> Vec<Type> {
        vec![]
    }

    fn ret() -> Type {
        Type::from::<T>()
    }
}

impl<T: Reflected, A0: Reflected> Reflected for fn(A0) -> T {
    fn name() -> String {
        format!("fn({}) -> {}", A0::name(), T::name())
    }

    fn assemble(_: Self::Meta, ptr: *mut ()) -> *mut Self {
        ptr as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        ((), self as *const Self as _)
    }

    unsafe fn init() {
        Type::new_fn::<fn(A0) -> T>()
    }
}

impl<T: Reflected, A0: Reflected> ReflectedFunction for fn(A0) -> T {
    fn args() -> Vec<Type> {
        vec![Type::from::<A0>()]
    }

    fn ret() -> Type {
        Type::from::<T>()
    }
}

// Never type
impl Reflected for ! {
    fn name() -> String {
        "!".into()
    }

    fn assemble(_: Self::Meta, ptr: *mut ()) -> *mut Self {
        ptr as _
    }

    fn disassemble(&self) -> (Self::Meta, *mut ()) {
        ((), self as *const Self as _)
    }

    unsafe fn init() {
        Type::new_prim::<!>()
    }
}
