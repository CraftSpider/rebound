//! Runtime information about a type

use crate::info::{AssocConst, AssocFn, Field, UnionField, Variant};
use crate::reflect::*;
use crate::{Error, Value};

use core::fmt;
use core::hash::{Hash, Hasher};
use std::any::TypeId;
use std::collections::BTreeMap;
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
                (self.vtable.as_mut)(RefHack(val))
            }
        }
    };
}

static REFLECTED_TYS: RwLock<BTreeMap<TypeId, Type>> = RwLock::new(BTreeMap::new());

/// Common information / operations between all types
pub trait CommonTypeInfo {
    /// Get this type's name
    fn name(&self) -> String;
    /// Get the known associated functions of this type
    fn assoc_fns(&self) -> &'static [AssocFn];
    /// Get the known associated constants of this type
    fn assoc_consts(&self) -> &'static [AssocConst];
    // fn impled_traits(&self) -> &'static [TraitInfo];

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
    as_mut: for<'a> fn(RefHack<'a, '_>) -> Result<Value<'a>, Error>,
}

impl fmt::Debug for TypeVTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TypeVTable {{ name: {:?}, assoc_fns: {:?}, assoc_consts: {:?}, as_ref: {:p}, as_mut: {:p} }}",
            self.name, self.assoc_fns, self.assoc_consts, self.as_ref as *const (), self.as_mut as *const (),
        )
    }
}

impl TypeVTable {
    const fn new<T: ?Sized + Reflected>() -> TypeVTable {
        TypeVTable {
            name: T::name,
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,

            as_ref: unsafe { core::mem::transmute::<fn(_) -> _, _>(T::Key::take_ref) },
            as_mut: unsafe { core::mem::transmute::<fn(_) -> _, _>(T::Key::take_mut) },
        }
    }
}

/// An enum that represents information about a reflected type. This supports basically any possible
/// type in Rust, including primitives, arrays, and references. Generally, the only requirement is
/// that the type implement the [`Reflected`] trait, though most types are also expected to
/// implement another trait related to information they possess not shared by other type kinds.
///
/// This is backed by [`TypeId`], through the usage of a `Key` associated type on all
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

/// Generate common functions for each of the variants of ty
macro_rules! ty_common {
    ($($var:ident),+) => {
        paste::paste! {
            $(
                #[doc = "Get this Type as a [`" $var "Info`], panicking on failure."]
                #[track_caller]
                pub const fn [<unwrap_ $var:snake>](&self) -> & [<$var Info>] {
                    if let Type::$var(info) = self {
                        info
                    } else {
                        panic!(concat!("Attempted to unwrap non-", stringify!($var:lower), " Type as ", stringify!($var:lower)))
                    }
                }

                #[doc = "Check whether this Type is a [`" $var "Info`]"]
                pub const fn [<is_ $var:snake>](&self) -> bool {
                    matches!(self, Type::$var(_))
                }
            )*
        }
    }
}

impl Type {
    fn add_ty_erased(ty: Type, id: TypeId) {
        let mut map = REFLECTED_TYS
            .write()
            .expect("REFLECTED_TYS not initialized correctly");

        map.entry(id).or_insert(ty);
    }

    /// Internal function used by generated code to initialize a Type for primitives
    #[doc(hidden)]
    pub(crate) const fn new_prim<T: ?Sized + Reflected>() -> Type {
        Type::Primitive(PrimitiveInfo {
            vtable: TypeVTable::new::<T>(),
        })
    }

    /// Internal function used by generated code to initialize a Type for tuples
    #[doc(hidden)]
    pub(crate) const fn new_tuple<T: ?Sized + ReflectedTuple>() -> Type {
        Type::Tuple(TupleInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        })
    }

    /// Internal function used by generated code to initialize a Type for arrays
    #[doc(hidden)]
    pub(crate) const fn new_array<T: ?Sized + ReflectedArray>() -> Type {
        Type::Array(ArrayInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            length: T::LENGTH,
        })
    }

    /// Internal function used by generated code to initialize a Type for slices
    #[doc(hidden)]
    pub(crate) const fn new_slice<T: ?Sized + ReflectedSlice>() -> Type {
        Type::Slice(SliceInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
        })
    }

    /// Internal function used by generated code to initialize a Type for pointers
    #[doc(hidden)]
    pub(crate) const fn new_ptr<T: ReflectedPointer>() -> Type {
        Type::Pointer(PointerInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            mutability: T::MUTABILITY,
        })
    }

    /// Internal function used by generated code to initialize a Type for references
    #[doc(hidden)]
    pub(crate) const fn new_ref<T: ReflectedReference>() -> Type {
        Type::Reference(ReferenceInfo {
            vtable: TypeVTable::new::<T>(),
            element: T::element,
            mutability: T::MUTABILITY,
        })
    }

    /// Internal function used by generated code to initialize a Type for function pointers
    #[doc(hidden)]
    pub(crate) const fn new_fn<T: ReflectedFunction>() -> Type {
        Type::Function(FunctionInfo {
            vtable: TypeVTable::new::<T>(),
            args: T::args,
            ret: T::ret,
        })
    }

    /// Internal function used by generated code to initialize a Type for structs
    pub const fn new_struct<T: ?Sized + ReflectedStruct>() -> Type {
        Type::Struct(StructInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        })
    }

    /// Internal function used by generated code to initialize a Type for tuple structs
    pub const fn new_tuple_struct<T: ReflectedTupleStruct>() -> Type {
        Type::TupleStruct(TupleStructInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        })
    }

    /// Internal function used by generated code to initialize a Type for unit structs
    pub const fn new_unit_struct<T: ReflectedUnitStruct>() -> Type {
        Type::UnitStruct(UnitStructInfo {
            vtable: TypeVTable::new::<T>(),
        })
    }

    /// Internal function used by generated code to initialize a Type for enums
    ///
    /// # Safety
    ///
    /// Should only be called inside a [`Reflected`] type's `init` impl
    pub const fn new_enum<T: ReflectedEnum>() -> Type {
        Type::Enum(EnumInfo {
            vtable: TypeVTable::new::<T>(),
            variants: T::variants,
        })
    }

    /// Internal function used by generated code to initialize a Type for unions
    ///
    /// # Safety
    ///
    /// Should only be called inside a [`Reflected`] type's `init` impl
    pub const fn new_union<T: ReflectedUnion>() -> Type {
        Type::Union(UnionInfo {
            vtable: TypeVTable::new::<T>(),
            fields: T::fields,
        })
    }

    /// Get a Type instance by name, assuming it has been instantiated beforehand.
    /// The name provided is expected to be of a certain normalized form, which may not
    /// be fully stable between versions. This function is also fairly slow.
    /// Prefer [`Type::of`] or [`Type::from_id`] if possible.
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
        let ref_tys = REFLECTED_TYS.read().unwrap();
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
            .read()
            .expect("Couldn't get read lock on Reflection mapping")
            .get(ty_id)
            .copied()
    }

    /// Make a type available via [`Type::from_id`] or [`Type::from_name`]
    pub fn initialize<T: ?Sized + Reflected>() {
        Type::add_ty_erased(T::TYPE, TypeId::of::<T::Key>())
    }

    /// Get a Type instance from any reflected type. This will not make it available via
    /// [`Type::from_id`] or [`Type::from_name`]
    pub const fn of<T: ?Sized + Reflected>() -> Type {
        T::TYPE
    }

    const fn as_inner(&self) -> &dyn CommonTypeInfo {
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

    ty_common!(
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
