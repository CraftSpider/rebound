use crate::{AssocFn, AssocConst, Type, Field};
use crate::reflect::*;
use crate::__helpers::*;

// TODO: Add impls for all these

// Integers
impl Reflected for u8 {
    fn name() -> String {
        "u8".into()
    }

    unsafe fn init() {
        Type::new_prim::<u8>()
    }
}

impl ReflectedImpl<0> for u8 {
    fn assoc_fns() -> Option<Vec<AssocFn>> {
        unsafe {
            Some(vec![
                AssocFn::new(
                    __make_static_helper!(u8::from_str_radix, &str, u32),
                    "from_str_radix",
                    Type::from::<u8>(),
                    None,
                    &[Type::from::<&str>(), Type::from::<u32>()],
                    Type::from::<Result<u8, core::num::ParseIntError>>()
                ),
            ])
        }
    }

    fn assoc_consts() -> Option<Vec<AssocConst>> {
        unsafe {
            Some(vec![
                AssocConst::new(
                    __make_const_accessor!(u8::MIN),
                    "MIN",
                    Type::from::<u8>(),
                    Type::from::<&u8>(),
                )
            ])
        }
    }
}

impl Reflected for u16 {
    fn name() -> String {
        "u16".into()
    }

    unsafe fn init() {
        Type::new_prim::<u16>()
    }
}

impl Reflected for u32 {
    fn name() -> String {
        "u32".into()
    }

    unsafe fn init() {
        Type::new_prim::<u32>()
    }
}

impl Reflected for u64 {
    fn name() -> String {
        "u64".into()
    }

    unsafe fn init() {
        Type::new_prim::<u64>()
    }
}

impl Reflected for u128 {
    fn name() -> String {
        "u128".into()
    }

    unsafe fn init() {
        Type::new_prim::<u128>()
    }
}

impl Reflected for usize {
    fn name() -> String {
        "usize".into()
    }

    unsafe fn init() {
        Type::new_prim::<usize>()
    }
}

impl Reflected for i8 {
    fn name() -> String {
        "i8".into()
    }

    unsafe fn init() {
        Type::new_prim::<i8>()
    }
}

impl Reflected for i16 {
    fn name() -> String {
        "i16".into()
    }

    unsafe fn init() {
        Type::new_prim::<i16>()
    }
}

impl Reflected for i32 {
    fn name() -> String {
        "i32".into()
    }

    unsafe fn init() {
        Type::new_prim::<i32>()
    }
}

impl Reflected for i64 {
    fn name() -> String {
        "i64".into()
    }

    unsafe fn init() {
        Type::new_prim::<i64>()
    }
}

impl Reflected for i128 {
    fn name() -> String {
        "i128".into()
    }

    unsafe fn init() {
        Type::new_prim::<i128>()
    }
}

impl Reflected for isize {
    fn name() -> String {
        "isize".into()
    }

    unsafe fn init() {
        Type::new_prim::<isize>()
    }
}

// Floats
impl Reflected for f32 {
    fn name() -> String {
        "f32".into()
    }

    unsafe fn init() {
        Type::new_prim::<f32>()
    }
}

impl Reflected for f64 {
    fn name() -> String {
        "f64".into()
    }

    unsafe fn init() {
        Type::new_prim::<f64>()
    }
}

// Other raw types
impl Reflected for bool {
    fn name() -> String {
        "bool".into()
    }

    unsafe fn init() {
        Type::new_prim::<bool>()
    }
}

impl Reflected for char {
    fn name() -> String {
        "char".into()
    }

    unsafe fn init() {
        Type::new_prim::<char>()
    }
}

impl Reflected for str {
    fn name() -> String {
        "str".into()
    }

    unsafe fn init() {
        Type::new_prim::<str>()
    }
}

// Tuple reflections
impl Reflected for () {
    fn name() -> String {
        "()".into()
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
impl<T0: Reflected + 'static> Reflected for (T0,) {
    fn name() -> String {
        format!("({},)", T0::name())
    }

    unsafe fn init() {
        Type::new_tuple::<(T0,)>()
    }
}

impl<T0: Reflected + 'static> ReflectedTuple for (T0,) {
    fn fields() -> Vec<Field> {
        unsafe {
            vec![
                Field::new_tuple(
                    __make_ref_accessor!((T0,), 0),
                    __make_setter!((T0,), 0),
                    0,
                    Type::from::<(T0,)>(),
                    Type::from::<T0>(),
                ),
            ]
        }
    }
}

impl<T0: Reflected + 'static, T1: Reflected + 'static> Reflected for (T0, T1) {
    fn name() -> String {
        format!("({}, {})", T0::name(), T1::name())
    }

    unsafe fn init() {
        Type::new_tuple::<(T0, T1)>()
    }
}

impl<T0: Reflected + 'static, T1: Reflected + 'static> ReflectedTuple for (T0, T1) {
    fn fields() -> Vec<Field> {
        unsafe {
            vec![
                Field::new_tuple(
                    __make_ref_accessor!((T0, T1), 0),
                    __make_setter!((T0, T1), 0),
                    0,
                    Type::from::<(T0, T1)>(),
                    Type::from::<T0>(),
                ),
                Field::new_tuple(
                    __make_ref_accessor!((T0, T1), 1),
                    __make_setter!((T0, T1), 1),
                    1,
                    Type::from::<(T0, T1)>(),
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
    fn name() -> String {
        format!("[{}]", T::name())
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

// Pointers
impl<T: Reflected> Reflected for *const T {
    fn name() -> String {
        format!("*const {}", T::name())
    }

    unsafe fn init() {
        Type::new_ptr::<*const T>();
    }
}

impl<T: Reflected> ReflectedPointer for *const T {
    fn element() -> Type {
        Type::from::<T>()
    }

    fn mutability() -> bool {
        false
    }
}

impl<T: Reflected> Reflected for *mut T {
    fn name() -> String {
        format!("*mut {}", T::name())
    }

    unsafe fn init() {
        Type::new_ptr::<*mut T>();
    }
}

impl<T: Reflected> ReflectedPointer for *mut T {
    fn element() -> Type {
        Type::from::<T>()
    }

    fn mutability() -> bool {
        true
    }
}

// References
impl<T: ?Sized + Reflected> Reflected for &T {
    fn name() -> String {
        format!("&{}", T::name())
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
    fn name() -> String {
        format!("&mut {}", T::name())
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

// Never type
#[cfg(feature = "never-type")]
impl Reflected for ! {
    fn name() -> String {
        "!".into()
    }

    unsafe fn init() {
        Type::new_prim::<!>()
    }
}
