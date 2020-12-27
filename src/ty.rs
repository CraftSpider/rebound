//! Runtime information about a type

use crate::info::*;
use crate::reflect::*;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::lazy::SyncOnceCell;
use std::sync::RwLock;

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
}

/// Common VTable used by all types
#[derive(Debug, Copy, Clone)]
struct TypeVTable {
    name: fn() -> String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,
}

impl TypeVTable {
    fn new<T: ?Sized + Reflected>() -> TypeVTable {
        TypeVTable {
            name: T::name,
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,
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
    Primitive(PrimitiveInfo),
    Tuple(TupleInfo),
    Array(ArrayInfo),
    Slice(SliceInfo),
    Pointer(PointerInfo),
    Reference(ReferenceInfo),
    Function(FunctionInfo),

    Struct(StructInfo),
    Enum(EnumInfo),
    TupleStruct(TupleStructInfo),
    UnitStruct(UnitStructInfo),
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

    #[doc(hidden)]
    pub unsafe fn new_prim<T: ?Sized + Reflected>() {
        let ty = Type::Primitive(PrimitiveInfo {
            vtable: TypeVTable::new::<T>(),
        });

        Type::add_ty(ty);
    }

    #[doc(hidden)]
    pub unsafe fn new_tuple<T: ?Sized + ReflectedTuple>() {
        let ty = Type::Tuple(TupleInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        });

        Type::add_ty(ty);
    }

    #[doc(hidden)]
    pub unsafe fn new_array<T: ?Sized + ReflectedArray>() {
        let ty = Type::Array(ArrayInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            length: T::length(),
        });

        Type::add_ty(ty);
    }

    #[doc(hidden)]
    pub unsafe fn new_slice<T: ?Sized + ReflectedSlice>() {
        let ty = Type::Slice(SliceInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
        });

        Type::add_ty(ty);
    }

    pub unsafe fn new_ptr<T: ReflectedPointer>() {
        let ty = Type::Pointer(PointerInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            mutability: T::mutability(),
        });

        Type::add_ty(ty);
    }

    pub unsafe fn new_ref<T: ReflectedReference>() {
        let ty = Type::Reference(ReferenceInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            mutability: T::mutability(),
        });

        Type::add_ty(ty);
    }

    pub unsafe fn new_fn<T: ReflectedFunction>() {
        let ty = Type::Function(FunctionInfo {
            vtable: TypeVTable::new::<T>(),
            args: T::args,
            ret: T::ret,
        });

        Type::add_ty(ty);
    }

    pub unsafe fn new_struct<T: ReflectedStruct>() {
        let ty = Type::Struct(StructInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        });

        Type::add_ty(ty);
    }

    pub unsafe fn new_enum<T: ReflectedEnum>() {
        let ty = Type::Enum(EnumInfo {
            vtable: TypeVTable::new::<T>(),
            variants: T::variants,
        });

        Type::add_ty(ty);
    }

    pub unsafe fn new_tuple_struct<T: ReflectedTupleStruct>() {
        let ty = Type::TupleStruct(TupleStructInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        });

        Type::add_ty(ty);
    }

    pub unsafe fn new_unit_struct<T: ReflectedUnitStruct>() {
        let ty = Type::UnitStruct(UnitStructInfo {
            vtable: TypeVTable::new::<T>(),
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
        // TODO: Multithreading support, so init isn't called multiple times
        unsafe {
            Type::from_name(&T::name()).unwrap_or_else(|| {
                T::init();

                Type::from_name(&T::name()).expect(&format!(
                    "Type for {} not initialized, despite calling T::init()",
                    T::name()
                ))
            })
        }
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
            Type::Enum(i) => i,
            Type::TupleStruct(i) => i,
            Type::UnitStruct(i) => i,
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
}

#[derive(Debug, Copy, Clone)]
pub struct PrimitiveInfo {
    vtable: TypeVTable,
}

impl CommonTypeInfo for PrimitiveInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}

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

impl CommonTypeInfo for TupleInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ArrayInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
    length: usize,
}

impl CommonTypeInfo for ArrayInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SliceInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
}

impl CommonTypeInfo for SliceInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PointerInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
    mutability: bool,
}

impl CommonTypeInfo for PointerInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ReferenceInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
    mutability: bool,
}

impl CommonTypeInfo for ReferenceInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FunctionInfo {
    vtable: TypeVTable,
    args: fn() -> Vec<Type>,
    ret: fn() -> Type,
}

impl CommonTypeInfo for FunctionInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}

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

impl CommonTypeInfo for StructInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}

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

impl CommonTypeInfo for EnumInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TupleStructInfo {
    vtable: TypeVTable,
    fields: fn() -> Vec<Field>,
}

impl CommonTypeInfo for TupleStructInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UnitStructInfo {
    vtable: TypeVTable,
}

impl CommonTypeInfo for UnitStructInfo {
    fn name(&self) -> String {
        (self.vtable.name)()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.vtable.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.vtable.assoc_consts)()
    }
}
