//! Runtime information about a type

use crate::info::*;
use crate::reflect::*;
use crate::utils::StaticTypeMap;
use crate::Value;

use core::fmt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::lazy::SyncOnceCell;
use std::sync::RwLock;

macro impl_common($ty:ty) {
    impl CommonTypeInfo for $ty {
        fn name(&self) -> String {
            (self.vtable.name)()
        }

        fn assoc_fns(&self) -> Vec<AssocFn> {
            (self.vtable.assoc_fns)()
        }

        fn assoc_consts(&self) -> Vec<AssocConst> {
            (self.vtable.assoc_consts)()
        }

        fn as_ref<'a>(&self, val: &'a Value) -> Value<'a> {
            (self.vtable.as_ref)(val)
        }

        fn as_mut<'a>(&self, val: &'a mut Value) -> Value<'a> {
            (self.vtable.as_mut)(val)
        }
    }
}

// SAFETY: *do not touch these if you don't know what you're doing*
static REFLECTED_TYS: SyncOnceCell<RwLock<HashMap<String, Type>>> = SyncOnceCell::new();

/// Common information / operations between all types
pub trait CommonTypeInfo {
    /// Get this type's name
    fn name(&self) -> String;
    /// Get the known associated functions of this type
    fn assoc_fns(&self) -> Vec<AssocFn>;
    /// Get the known associated constants of this type
    fn assoc_consts(&self) -> Vec<AssocConst>;
    // fn impled_traits(&self) -> Vec<TraitInfo>;

    /// Convert a Value of this type to a reference to that value, if it's not already a reference
    fn as_ref<'a>(&self, val: &'a Value) -> Value<'a>;
    /// Convert a Value of this type to a mutable reference to that value, if it's not already a
    /// reference
    fn as_mut<'a>(&self, val: &'a mut Value) -> Value<'a>;
}

/// Common VTable used by all types
#[derive(Copy, Clone)]
struct TypeVTable {
    name: fn() -> String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,

    as_ref: for<'a> fn(&'a Value) -> Value<'a>,
    as_mut: for<'a> fn(&'a mut Value) -> Value<'a>,
}

impl fmt::Debug for TypeVTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TypeVTable {{ name: {:?}, assoc_fns: {:?}, assoc_consts: {:?}, as_ref: {:p} }}",
            self.name, self.assoc_fns, self.assoc_consts, self.as_ref as *const ()
        )
    }
}

trait Ref: Reflected {
    fn as_ref<'a>(val: &'a Value) -> Value<'a>;
    fn as_mut<'a>(val: &'a mut Value) -> Value<'a>;
}

impl<T: ?Sized + Reflected> Ref for T {
    default fn as_ref<'a>(val: &'a Value) -> Value<'a> {
        unsafe { std::mem::transmute(Value::from(val.borrow::<Self>())) }
    }

    default fn as_mut<'a>(val: &'a mut Value) -> Value<'a> {
        unsafe { std::mem::transmute(Value::from(val.borrow_mut::<Self>())) }
    }
}

impl<T: ?Sized + Reflected> Ref for &T {
    fn as_ref<'a>(val: &'a Value) -> Value<'a> {
        unsafe {
            let ptr = &*<&T>::assemble(*val.raw_meta().cast(), val.raw_ptr().cast());
            std::mem::transmute(Value::from_ref(ptr))
        }
    }

    fn as_mut<'a>(_: &'a mut Value) -> Value<'a> {
        panic!("Can't mutably borrow an immutable reference")
    }
}

impl<T: ?Sized + Reflected> Ref for &mut T {
    fn as_ref<'a>(val: &'a Value) -> Value<'a> {
        unsafe {
            let ptr = &*<&T>::assemble(*val.raw_meta().cast(), val.raw_ptr().cast());
            std::mem::transmute(Value::from_ref(ptr))
        }
    }

    fn as_mut<'a>(val: &'a mut Value) -> Value<'a> {
        unsafe {
            let ptr = &*<&mut T>::assemble(*val.raw_meta().cast(), val.raw_ptr().cast());
            std::mem::transmute(Value::from_ref(ptr))
        }
    }
}

impl TypeVTable {
    fn new<T: ?Sized + Reflected>() -> TypeVTable {
        TypeVTable {
            name: T::name,
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,

            as_ref: <T as Ref>::as_ref,
            as_mut: <T as Ref>::as_mut,
        }
    }
}

/// An enum that represents information about a reflected type. This supports basically any possible
/// type in Rust, including primitives, arrays, and references. Generally, the only requirement is
/// that the type implement the [`Reflected`] trait, though most types are also expected to
/// implement another trait related to information they possess not shared by other type kinds.
///
/// This is not, and cannot, be backed by [`core::any::TypeId`], because that is only valid on
/// `'static` types, while this works with dynamic lifetimes.
#[derive(Debug, Copy, Clone)]
pub enum Type {
    /// A primitive simple type, such as `u8` or `str`
    Primitive(PrimitiveInfo),
    /// A tuple type, `(T0, .., Tn)`
    Tuple(TupleInfo),
    /// An array type, `[T; N]`
    Array(ArrayInfo),
    /// A slice type, `[T]`
    Slice(SliceInfo),
    /// A pointer type, either `*const T` or `*mut T`
    Pointer(PointerInfo),
    /// A reference type, either `&T` or `&mut T`
    Reference(ReferenceInfo),
    /// A function pointer type, `fn(T1..Tn) -> T0`
    Function(FunctionInfo),

    /// A struct type, with named fields
    Struct(StructInfo),
    /// A struct type, with unnamed fields
    TupleStruct(TupleStructInfo),
    /// A struct type, with no fields
    UnitStruct(UnitStructInfo),
    /// An enum type, with variants
    Enum(EnumInfo),
}

impl Type {
    fn add_ty(ty: Type) {
        let mut map = REFLECTED_TYS
            .get_or_init(|| RwLock::new(HashMap::new()))
            .write()
            .expect("REFLECTED_TYS not initialized correctly");

        let name = ty.name();

        if map.contains_key(&name) {
            eprintln!("Type {} already registered", name);
        }

        map.insert(name, ty);
    }

    /// Internal function used by generated code to initialize a Type for primitives
    #[doc(hidden)]
    pub unsafe fn new_prim<T: ?Sized + Reflected>() {
        let ty = Type::Primitive(PrimitiveInfo {
            vtable: TypeVTable::new::<T>(),
        });

        Type::add_ty(ty);
    }

    /// Internal function used by generated code to initialize a Type for tuples
    #[doc(hidden)]
    pub unsafe fn new_tuple<T: ?Sized + ReflectedTuple>() {
        let ty = Type::Tuple(TupleInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        });

        Type::add_ty(ty);
    }

    /// Internal function used by generated code to initialize a Type for arrays
    #[doc(hidden)]
    pub unsafe fn new_array<T: ?Sized + ReflectedArray>() {
        let ty = Type::Array(ArrayInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            length: T::length(),
        });

        Type::add_ty(ty);
    }

    /// Internal function used by generated code to initialize a Type for slices
    #[doc(hidden)]
    pub unsafe fn new_slice<T: ?Sized + ReflectedSlice>() {
        let ty = Type::Slice(SliceInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
        });

        Type::add_ty(ty);
    }

    /// Internal function used by generated code to initialize a Type for pointers
    #[doc(hidden)]
    pub unsafe fn new_ptr<T: ReflectedPointer>() {
        let ty = Type::Pointer(PointerInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            mutability: T::mutability(),
        });

        Type::add_ty(ty);
    }

    /// Internal function used by generated code to initialize a Type for references
    #[doc(hidden)]
    pub unsafe fn new_ref<T: ReflectedReference>() {
        let ty = Type::Reference(ReferenceInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            mutability: T::mutability(),
        });

        Type::add_ty(ty);
    }

    /// Internal function used by generated code to initialize a Type for function pointers
    #[doc(hidden)]
    pub unsafe fn new_fn<T: ReflectedFunction>() {
        let ty = Type::Function(FunctionInfo {
            vtable: TypeVTable::new::<T>(),
            args: T::args,
            ret: T::ret,
        });

        Type::add_ty(ty);
    }

    /// Internal function used by generated code to initialize a Type for structs
    pub unsafe fn new_struct<T: ReflectedStruct>() {
        let ty = Type::Struct(StructInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        });

        Type::add_ty(ty);
    }

    /// Internal function used by generated code to initialize a Type for tuple structs
    pub unsafe fn new_tuple_struct<T: ReflectedTupleStruct>() {
        let ty = Type::TupleStruct(TupleStructInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        });

        Type::add_ty(ty);
    }

    /// Internal function used by generated code to initialize a Type for unit structs
    pub unsafe fn new_unit_struct<T: ReflectedUnitStruct>() {
        let ty = Type::UnitStruct(UnitStructInfo {
            vtable: TypeVTable::new::<T>(),
        });

        Type::add_ty(ty);
    }

    /// Internal function used by generated code to initialize a Type for enums
    pub unsafe fn new_enum<T: ReflectedEnum>() {
        let ty = Type::Enum(EnumInfo {
            vtable: TypeVTable::new::<T>(),
            variants: T::variants,
        });

        Type::add_ty(ty);
    }

    /// Get a Type instance by name, assuming it has been instantiated beforehand.
    /// The name provided is expected to be of a certain normalized form, which may not
    /// be fully stable between versions. Prefer [`Type::from`] if possible.
    ///
    /// Current Requirements:
    /// - All struct names should be fully qualified, so for example the Type for Type would be
    ///   `rebound::ty::Type`
    /// - Any commas will be followed by spaces, and there will be no trailing commas except in the
    ///   case of 1-element tuples
    /// - References will have no lifetime
    /// - Possibly other things
    ///
    /// # Safety
    ///
    /// This function is in no way memory unsafe, however, the format used for type names is an
    /// implementation detail, and thus may change even across patch versions.
    pub unsafe fn from_name(name: &str) -> Option<Type> {
        REFLECTED_TYS
            .get_or_init(|| RwLock::new(HashMap::new()))
            .read()
            .expect("Couldn't get read lock on Reflection mapping")
            .get(name)
            .map(|t| *t)
    }

    /// Get a Type instance from any reflected type, instantiating it if necessary.
    pub fn from<T: ?Sized + Reflected>() -> Type {
        static INIT: SyncOnceCell<StaticTypeMap<()>> = SyncOnceCell::new();
        INIT.get_or_init(|| StaticTypeMap::new())
            .call_once::<T, _>(|| {
                unsafe { T::init() };
            });

        unsafe { Type::from_name(&T::name()).expect("Type not initialized") }
    }

    fn as_inner(&self) -> &dyn CommonTypeInfo {
        match self {
            Type::Primitive(i) => i,
            Type::Tuple(i) => i,
            Type::Slice(i) => i,
            Type::Array(i) => i,
            Type::Pointer(i) => i,
            Type::Reference(i) => i,
            Type::Function(i) => i,

            Type::Struct(i) => i,
            Type::TupleStruct(i) => i,
            Type::UnitStruct(i) => i,
            Type::Enum(i) => i,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        // This is safe because Type creation is based on the name, overlaps will cause warnings
        return self.name() == other.name();
    }
}

impl Eq for Type {}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl CommonTypeInfo for Type {
    fn name(&self) -> String {
        self.as_inner().name()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        self.as_inner().assoc_fns()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        self.as_inner().assoc_consts()
    }

    fn as_ref<'a>(&self, val: &'a Value) -> Value<'a> {
        self.as_inner().as_ref(val)
    }

    fn as_mut<'a>(&self, val: &'a mut Value) -> Value<'a> {
        self.as_inner().as_mut(val)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PrimitiveInfo {
    vtable: TypeVTable,
}

impl_common!(PrimitiveInfo);

#[derive(Debug, Copy, Clone)]
pub struct TupleInfo {
    vtable: TypeVTable,
    fields: fn() -> Vec<Field>,
}

impl TupleInfo {
    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }
}

impl_common!(TupleInfo);

#[derive(Debug, Copy, Clone)]
pub struct ArrayInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
    length: usize,
}

impl ArrayInfo {
    pub fn element(&self) -> Type {
        (self.element)()
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

impl_common!(ArrayInfo);

#[derive(Debug, Copy, Clone)]
pub struct SliceInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
}

impl SliceInfo {
    pub fn element(&self) -> Type {
        (self.element)()
    }
}

impl_common!(SliceInfo);

#[derive(Debug, Copy, Clone)]
pub struct PointerInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
    mutability: bool,
}

impl PointerInfo {
    pub fn element(&self) -> Type {
        (self.element)()
    }

    pub fn mutability(&self) -> bool {
        self.mutability
    }
}

impl_common!(PointerInfo);

#[derive(Debug, Copy, Clone)]
pub struct ReferenceInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
    mutability: bool,
}

impl ReferenceInfo {
    pub fn element(&self) -> Type {
        (self.element)()
    }

    pub fn mutability(&self) -> bool {
        self.mutability
    }
}

impl_common!(ReferenceInfo);

#[derive(Debug, Copy, Clone)]
pub struct FunctionInfo {
    vtable: TypeVTable,
    args: fn() -> Vec<Type>,
    ret: fn() -> Type,
}

impl FunctionInfo {
    pub fn arg_tys(&self) -> Vec<Type> {
        (self.args)()
    }

    pub fn ret_ty(&self) -> Type {
        (self.ret)()
    }
}

impl_common!(FunctionInfo);

#[derive(Debug, Copy, Clone)]
pub struct StructInfo {
    vtable: TypeVTable,
    fields: fn() -> Vec<Field>,
}

impl StructInfo {
    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }
}

impl_common!(StructInfo);

#[derive(Debug, Copy, Clone)]
pub struct TupleStructInfo {
    vtable: TypeVTable,
    fields: fn() -> Vec<Field>,
}

impl TupleStructInfo {
    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }
}

impl_common!(TupleStructInfo);

#[derive(Debug, Copy, Clone)]
pub struct UnitStructInfo {
    vtable: TypeVTable,
}

impl_common!(UnitStructInfo);

#[derive(Debug, Copy, Clone)]
pub struct EnumInfo {
    vtable: TypeVTable,
    variants: fn() -> Vec<Variant>,
}

impl EnumInfo {
    pub fn variants(&self) -> Vec<Variant> {
        (self.variants)()
    }
}

impl_common!(EnumInfo);
