//! Runtime information about a type

use crate::info::{AssocConst, AssocFn, Field, UnionField, Variant};
use crate::reflect::*;
use crate::utils::StaticTypeMap;
use crate::{Error, Value};

use core::fmt;
use core::hash::{Hash, Hasher};
use once_cell::sync::OnceCell;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::RwLock;

/// Implement `CommonTypeInfo` for a given struct
macro_rules! impl_common {
    ($ty:ty) => {
        impl CommonTypeInfo for $ty {
            fn name(&self) -> String {
                (self.vtable.name)()
            }

            fn assoc_fns(&self) -> &'static [AssocFn] {
                (self.vtable.assoc_fns)()
            }

            fn assoc_consts(&self) -> &'static [AssocConst] {
                (self.vtable.assoc_consts)()
            }

            fn as_ref<'a>(&self, val: &'a Value<'_>) -> Result<Value<'a>, Error> {
                (self.vtable.as_ref)(val)
            }

            fn as_mut<'a>(&self, val: &'a mut Value<'_>) -> Result<Value<'a>, Error> {
                (self.vtable.as_mut)(val)
            }
        }
    };
}

// SAFETY: *do not touch these if you don't know what you're doing*
static REFLECTED_TYS: OnceCell<RwLock<HashMap<TypeId, Type>>> = OnceCell::new();

/// Common information / operations between all types
pub trait CommonTypeInfo {
    /// Get this type's name
    fn name(&self) -> String;
    /// Get the known associated functions of this type
    fn assoc_fns(&self) -> &'static [AssocFn];
    /// Get the known associated constants of this type
    fn assoc_consts(&self) -> &'static [AssocConst];
    // fn impled_traits(&self) -> Vec<TraitInfo>;

    /// Convert a Value of this type to a reference to that value, if it's not already a reference
    fn as_ref<'a>(&self, val: &'a Value<'_>) -> Result<Value<'a>, Error>;
    /// Convert a Value of this type to a mutable reference to that value, if it's not already a
    /// reference
    fn as_mut<'a>(&self, val: &'a mut Value<'_>) -> Result<Value<'a>, Error>;
}

/// Common `VTable` used by all types
#[derive(Copy, Clone)]
struct TypeVTable {
    name: fn() -> String,
    assoc_fns: fn() -> &'static [AssocFn],
    assoc_consts: fn() -> &'static [AssocConst],

    as_ref: for<'a> fn(&'a Value<'_>) -> Result<Value<'a>, Error>,
    as_mut: for<'a> fn(&'a mut Value<'_>) -> Result<Value<'a>, Error>,
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

impl TypeVTable {
    fn new<T: ?Sized + Reflected>() -> TypeVTable {
        TypeVTable {
            name: T::name,
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,

            as_ref: <T as Ref>::ref_val,
            as_mut: <T as Ref>::mut_val,
        }
    }
}

/// An enum that represents information about a reflected type. This supports basically any possible
/// type in Rust, including primitives, arrays, and references. Generally, the only requirement is
/// that the type implement the [`Reflected`] trait, though most types are also expected to
/// implement another trait related to information they possess not shared by other type kinds.
///
/// This is backed by [`core::any::TypeId`], through the usage of a `Key` associated type on all
/// Reflected items, which represents a `'static` version of that item, even if the item isn't
/// always static. This works because lifetimes are erased for Type instances anyways.
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
    /// A union type, with fields
    Union(UnionInfo),
}

/// Generate unwrap functions for each of the variants of ty
macro_rules! ty_unwraps {
    ($($var:ident),+) => {
        paste::paste! {
            $(
                #[doc = "Get this Type as a [`" $var "Info`], panicking on failure."]
                #[track_caller]
                pub fn [<unwrap_ $var:snake>](&self) -> & [<$var Info>] {
                    if let Type::$var(info) = self {
                        info
                    } else {
                        panic!(concat!("Attempted to unwrap non-", stringify!($var:lower), " Type as ", stringify!($var:lower)))
                    }
                }
            )*
        }
    }
}

impl Type {
    fn add_ty<T: ?Sized + Reflected>(ty: Type) {
        let mut map = REFLECTED_TYS
            .get_or_init(|| RwLock::new(HashMap::new()))
            .write()
            .expect("REFLECTED_TYS not initialized correctly");

        let type_id = TypeId::of::<T::Key>();

        if map.contains_key(&type_id) {
            panic!("Type {} already registered", ty.name());
        }

        map.insert(type_id, ty);
    }

    /// Internal function used by generated code to initialize a Type for primitives
    #[doc(hidden)]
    pub unsafe fn new_prim<T: ?Sized + Reflected>() {
        let ty = Type::Primitive(PrimitiveInfo {
            vtable: TypeVTable::new::<T>(),
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for tuples
    #[doc(hidden)]
    pub unsafe fn new_tuple<T: ?Sized + ReflectedTuple>() {
        let ty = Type::Tuple(TupleInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for arrays
    #[doc(hidden)]
    pub unsafe fn new_array<T: ?Sized + ReflectedArray>() {
        let ty = Type::Array(ArrayInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            length: T::length(),
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for slices
    #[doc(hidden)]
    pub unsafe fn new_slice<T: ?Sized + ReflectedSlice>() {
        let ty = Type::Slice(SliceInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for pointers
    #[doc(hidden)]
    pub unsafe fn new_ptr<T: ReflectedPointer>() {
        let ty = Type::Pointer(PointerInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            mutability: T::mutability(),
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for references
    #[doc(hidden)]
    pub unsafe fn new_ref<T: ReflectedReference>() {
        let ty = Type::Reference(ReferenceInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            mutability: T::mutability(),
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for function pointers
    #[doc(hidden)]
    pub unsafe fn new_fn<T: ReflectedFunction>() {
        let ty = Type::Function(FunctionInfo {
            vtable: TypeVTable::new::<T>(),
            args: T::args,
            ret: T::ret,
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for structs
    ///
    /// # Safety
    ///
    /// Should only be called inside a [`Reflected`] type's `init` impl
    pub unsafe fn new_struct<T: ?Sized + ReflectedStruct>() {
        let ty = Type::Struct(StructInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for tuple structs
    ///
    /// # Safety
    ///
    /// Should only be called inside a [`Reflected`] type's `init` impl
    pub unsafe fn new_tuple_struct<T: ReflectedTupleStruct>() {
        let ty = Type::TupleStruct(TupleStructInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for unit structs
    ///
    /// # Safety
    ///
    /// Should only be called inside a [`Reflected`] type's `init` impl
    pub unsafe fn new_unit_struct<T: ReflectedUnitStruct>() {
        let ty = Type::UnitStruct(UnitStructInfo {
            vtable: TypeVTable::new::<T>(),
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for enums
    ///
    /// # Safety
    ///
    /// Should only be called inside a [`Reflected`] type's `init` impl
    pub unsafe fn new_enum<T: ReflectedEnum>() {
        let ty = Type::Enum(EnumInfo {
            vtable: TypeVTable::new::<T>(),
            variants: T::variants,
        });

        Type::add_ty::<T>(ty);
    }

    /// Internal function used by generated code to initialize a Type for unions
    ///
    /// # Safety
    ///
    /// Should only be called inside a [`Reflected`] type's `init` impl
    pub unsafe fn new_union<T: ReflectedUnion>() {
        let ty = Type::Union(UnionInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        });

        Type::add_ty::<T>(ty);
    }

    /// Get a Type instance by name, assuming it has been instantiated beforehand.
    /// The name provided is expected to be of a certain normalized form, which may not
    /// be fully stable between versions. This function is also fairly slow.
    /// Prefer [`Type::from`] or [`Type::from_id`] if possible.
    ///
    /// Current Requirements:
    /// - All struct names should be fully qualified, so for example the Type for Type would be
    ///   `rebound::ty::Type`
    /// - Any commas will be followed by spaces, and there will be no trailing commas except in the
    ///   case of 1-element tuples
    /// - References will have no lifetime
    /// - Possibly other things
    ///
    /// # Panics
    ///
    /// If the function fails to acquire the global reflection lock
    ///
    /// # Safety
    ///
    /// This function is in no way memory unsafe, however, the format used for type names is an
    /// implementation detail, and thus may change even across patch versions.
    pub unsafe fn from_name(name: &str) -> Option<Type> {
        let ref_tys = REFLECTED_TYS
            .get_or_init(|| RwLock::new(HashMap::new()))
            .read()
            .unwrap();
        for ty in ref_tys.values() {
            if ty.name() == name {
                return Some(*ty);
            }
        }

        None
    }

    /// Get a [`Type`] instance by [`TypeId`] of its associated key, assuming it has been instantiated
    /// beforehand.
    pub fn from_id(ty_id: &TypeId) -> Option<Type> {
        REFLECTED_TYS
            .get_or_init(|| RwLock::new(HashMap::new()))
            .read()
            .expect("Couldn't get read lock on Reflection mapping")
            .get(ty_id)
            .copied()
    }

    /// Get a Type instance from any reflected type, instantiating it if necessary.
    pub fn from<T: ?Sized + Reflected>() -> Type {
        static INIT: OnceCell<StaticTypeMap<()>> = OnceCell::new();
        INIT.get_or_init(StaticTypeMap::new).call_once::<T, _>(|| {
            unsafe { T::init() };
        });

        Type::from_id(&TypeId::of::<T::Key>()).expect("Type not initialized")
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
            Type::Union(i) => i,
        }
    }

    ty_unwraps!(
        Primitive,
        Tuple,
        Array,
        Slice,
        Pointer,
        Reference,
        Function,
        Struct,
        TupleStruct,
        UnitStruct,
        Enum,
        Union
    );
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        // This is safe because Type creation is based on the name, overlaps will cause warnings
        self.name() == other.name()
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

    fn assoc_fns(&self) -> &'static [AssocFn] {
        self.as_inner().assoc_fns()
    }

    fn assoc_consts(&self) -> &'static [AssocConst] {
        self.as_inner().assoc_consts()
    }

    fn as_ref<'a>(&self, val: &'a Value<'_>) -> Result<Value<'a>, Error> {
        self.as_inner().as_ref(val)
    }

    fn as_mut<'a>(&self, val: &'a mut Value<'_>) -> Result<Value<'a>, Error> {
        self.as_inner().as_mut(val)
    }
}

/// Type information about a primitive reflected type
#[derive(Debug, Copy, Clone)]
pub struct PrimitiveInfo {
    vtable: TypeVTable,
}

impl_common!(PrimitiveInfo);

/// Type information about a reflected tuple
#[derive(Debug, Copy, Clone)]
pub struct TupleInfo {
    vtable: TypeVTable,
    fields: fn() -> &'static [Field],
}

impl TupleInfo {
    /// Get all the [`Fields`](Field) of this Tuple
    pub fn fields(&self) -> &'static [Field] {
        (self.fields)()
    }
}

impl_common!(TupleInfo);

/// Type information about a reflected array
#[derive(Debug, Copy, Clone)]
pub struct ArrayInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
    length: usize,
}

impl ArrayInfo {
    /// Get the element [`Type`] of this Array
    pub fn element(&self) -> Type {
        (self.element)()
    }

    /// Get the length of this Array
    pub fn length(&self) -> usize {
        self.length
    }
}

impl_common!(ArrayInfo);

/// Type information about a reflected slice
#[derive(Debug, Copy, Clone)]
pub struct SliceInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
}

impl SliceInfo {
    /// Get the element [`Type`] of this Slice
    pub fn element(&self) -> Type {
        (self.element)()
    }
}

impl_common!(SliceInfo);

/// Type information about a reflected pointer
#[derive(Debug, Copy, Clone)]
pub struct PointerInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
    mutability: bool,
}

impl PointerInfo {
    /// Get the element [`Type`] of this Pointer
    pub fn element(&self) -> Type {
        (self.element)()
    }

    /// Check whether this pointer is mutable
    pub fn mutability(&self) -> bool {
        self.mutability
    }
}

impl_common!(PointerInfo);

/// Type information about a reflected reference
#[derive(Debug, Copy, Clone)]
pub struct ReferenceInfo {
    vtable: TypeVTable,
    element: fn() -> Type,
    mutability: bool,
}

impl ReferenceInfo {
    /// Get the element [`Type`] of this Reference
    pub fn element(&self) -> Type {
        (self.element)()
    }

    /// Check whether this reference is mutable
    pub fn mutability(&self) -> bool {
        self.mutability
    }
}

impl_common!(ReferenceInfo);

/// Type information about a reflected function pointer
#[derive(Debug, Copy, Clone)]
pub struct FunctionInfo {
    vtable: TypeVTable,
    args: fn() -> Vec<Type>,
    ret: fn() -> Type,
}

impl FunctionInfo {
    /// Get the argument [`Types`](Type) of this Function
    pub fn arg_tys(&self) -> Vec<Type> {
        (self.args)()
    }

    /// Get the return [`Type`] of this Function
    pub fn ret_ty(&self) -> Type {
        (self.ret)()
    }
}

impl_common!(FunctionInfo);

/// Type information about a reflected struct
#[derive(Debug, Copy, Clone)]
pub struct StructInfo {
    vtable: TypeVTable,
    fields: fn() -> Vec<Field>,
}

impl StructInfo {
    /// Get all the [`Fields`](Field) of this Struct
    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }
}

impl_common!(StructInfo);

/// Type information about a reflected tuple-struct
#[derive(Debug, Copy, Clone)]
pub struct TupleStructInfo {
    vtable: TypeVTable,
    fields: fn() -> Vec<Field>,
}

impl TupleStructInfo {
    /// Get all the [`Fields`](Field) of this Tuple Struct
    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }
}

impl_common!(TupleStructInfo);

/// Type information about a reflected unit struct
#[derive(Debug, Copy, Clone)]
pub struct UnitStructInfo {
    vtable: TypeVTable,
}

impl_common!(UnitStructInfo);

/// Type information about a reflected enum
#[derive(Debug, Copy, Clone)]
pub struct EnumInfo {
    vtable: TypeVTable,
    variants: fn() -> Vec<Variant>,
}

impl EnumInfo {
    /// Get all the [`Variants`](Variant) of this Enum
    pub fn variants(&self) -> Vec<Variant> {
        (self.variants)()
    }

    /// Retrieve the [`Variant`] of a given [`Value`]
    pub fn variant_of(&self, val: &Value<'_>) -> Result<Variant, Error> {
        for i in self.variants() {
            if i.is_variant(val)? {
                return Ok(i);
            }
        }
        Err(Error::wrong_type(val.ty(), Type::Enum(*self)))
    }

    /// Check whether a [`Value`] is of a specific [`Variant`], if it's of this type
    pub fn is_variant(&self, val: &Value<'_>, variant: &Variant) -> Result<bool, Error> {
        if variant.assoc_ty() == Type::Enum(*self) {
            variant.is_variant(val)
        } else {
            Err(Error::wrong_type(variant.assoc_ty(), Type::Enum(*self)))
        }
    }
}

impl_common!(EnumInfo);

/// Type information about a reflected union
#[derive(Debug, Copy, Clone)]
pub struct UnionInfo {
    vtable: TypeVTable,
    fields: fn() -> Vec<UnionField>,
}

impl UnionInfo {
    /// Get all the [`Fields`](Field) of this Union
    pub fn fields(&self) -> Vec<UnionField> {
        (self.fields)()
    }
}

impl_common!(UnionInfo);
