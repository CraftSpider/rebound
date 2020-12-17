// #[allow(unused_imports)]
// use crate::prelude::*;

use crate::info::*;
use crate::reflect::*;

use std::collections::HashMap;
use std::sync::RwLock;

// SAFETY: *do not touch this*
static REFLECTED_TYS: RwLock<HashMap<String, Box<TypeInfo>>> = RwLock::new(HashMap::new());
// static mut REFLECTED_TYS: Option<HashMap<String, Box<TypeInfo>>> = None;

pub type Type = &'static TypeInfo;

/// Common things between all types
pub trait CommonTypeInfo {
    fn name(&self) -> &str;
    fn assoc_fns(&self) -> Vec<AssocFn>;
    fn assoc_consts(&self) -> Vec<AssocConst>;
    // fn impled_traits(&self) -> Vec<TraitInfo>;
}


/// An enum that represents information about a reflected type
#[derive(Debug)]
pub enum TypeInfo {
    Primitive(PrimitiveInfo),
    Tuple(TupleInfo),
    Array(ArrayInfo),
    Slice(SliceInfo),
    Pointer(PointerInfo),
    Reference(ReferenceInfo),

    Struct(StructInfo),
    Enum(EnumInfo),
    TupleStruct(TupleStructInfo),
    UnitStruct(UnitStructInfo),
}

impl TypeInfo {
    fn ensure_statics() {
        unsafe {
            if let None = REFLECTED_TYS {
                REFLECTED_TYS = Some(HashMap::new())
            }
        }
    }

    fn add_ty(ty: TypeInfo) {
        TypeInfo::ensure_statics();

        let map;
        unsafe {
            map = REFLECTED_TYS
                .as_mut()
                .expect("REFLECTED_TYS not initialized correctly");
        }

        let name = ty.name().to_string();

        if map.contains_key(&name) {
            panic!("Type {} already registered", name);
        }

        map.insert(name, Box::new(ty));
    }

    pub unsafe fn new_prim<T: ?Sized + Reflected>() {
        let ty = TypeInfo::Primitive(PrimitiveInfo {
            name: T::name(),
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,
        });

        TypeInfo::add_ty(ty);
    }

    pub unsafe fn new_tuple<T: ReflectedTuple>() {
        let ty = TypeInfo::Tuple(TupleInfo {
            name: T::name(),
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,
            fields: T::fields,
        });

        TypeInfo::add_ty(ty);
    }

    pub unsafe fn new_array<T: ?Sized + ReflectedArray>() {
        let ty = TypeInfo::Array(ArrayInfo {
            name: T::name(),
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,

            element: T::element(),
            length: T::length(),
        });

        TypeInfo::add_ty(ty);
    }

    pub unsafe fn new_slice<T: ?Sized + ReflectedSlice>() {
        let ty = TypeInfo::Slice(SliceInfo {
            name: T::name(),
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,

            element: T::element(),
        });

        TypeInfo::add_ty(ty);
    }

    pub unsafe fn new_ptr<T: ReflectedPointer>() {
        let ty = TypeInfo::Pointer(PointerInfo {
            name: T::name(),
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,

            element: T::element(),
            mutability: T::mutability(),
        });

        TypeInfo::add_ty(ty);
    }

    pub unsafe fn new_ref<T: ReflectedReference>() {
        let ty = TypeInfo::Reference(ReferenceInfo {
            name: T::name(),
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,

            element: T::element(),
            mutability: T::mutability(),
        });

        TypeInfo::add_ty(ty);
    }

    pub unsafe fn new_struct<T: ReflectedStruct>() {
        let ty = TypeInfo::Struct(StructInfo {
            name: T::name(),
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,
            fields: T::fields,
        });

        TypeInfo::add_ty(ty);
    }

    pub unsafe fn new_enum<T: ReflectedEnum>() {
        let ty = TypeInfo::Enum(EnumInfo {
            name: T::name(),
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,
            variants: T::variants,
        });

        TypeInfo::add_ty(ty);
    }

    pub unsafe fn new_tuple_struct<T: ReflectedTupleStruct>() {
        let ty = TypeInfo::TupleStruct(TupleStructInfo {
            name: T::name(),
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,
            fields: T::fields
        });

        TypeInfo::add_ty(ty);
    }

    pub unsafe fn new_unit_struct<T: ReflectedUnitStruct>() {
        let ty = TypeInfo::UnitStruct(UnitStructInfo {
            name: T::name(),
            assoc_fns: T::assoc_fns,
            assoc_consts: T::assoc_consts,
        });

        TypeInfo::add_ty(ty);
    }

    pub fn from_name(name: &str) -> Option<Type> {
        TypeInfo::ensure_statics();

        unsafe {
            REFLECTED_TYS
                .as_ref()
                .expect("static Reflection mapping not generated")
                .get(name)
                .map(|b| b.as_ref())
        }
    }

    pub fn from<T: ?Sized + Reflected>() -> Type {
        TypeInfo::from_name(&T::name())
            .unwrap_or_else(|| {
                unsafe { T::init() }

                TypeInfo::from_name(&T::name()).expect(&format!("TypeInfo for {} not initialized, despite calling T::init()", T::name()))
            })
    }

    fn as_inner(&self) -> &dyn CommonTypeInfo {
        match self {
            TypeInfo::Primitive(i) => i,
            TypeInfo::Tuple(i) => i,
            TypeInfo::Slice(i) => i,
            TypeInfo::Array(i) => i,
            TypeInfo::Pointer(i) => i,
            TypeInfo::Reference(i) => i,

            TypeInfo::Struct(i) => i,
            TypeInfo::Enum(i) => i,
            TypeInfo::TupleStruct(i) => i,
            TypeInfo::UnitStruct(i) => i,
        }
    }
}

impl PartialEq for TypeInfo {
    fn eq(&self, other: &TypeInfo) -> bool {
        // TODO: May be possible to break this assumption
        return self.name() == other.name()
    }
}

impl CommonTypeInfo for TypeInfo {
    fn name(&self) -> &str {
        self.as_inner().name()
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        self.as_inner().assoc_fns()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        self.as_inner().assoc_consts()
    }
}

#[derive(Debug)]
pub struct PrimitiveInfo {
    name: String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,
}

impl CommonTypeInfo for PrimitiveInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.assoc_consts)()
    }
}

#[derive(Debug)]
pub struct TupleInfo {
    name: String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,

    fields: fn() -> Vec<TupleField>,
}

impl TupleInfo {
    pub fn fields(&self) -> Vec<TupleField> {
        (self.fields)()
    }
}

impl CommonTypeInfo for TupleInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.assoc_consts)()
    }
}

#[derive(Debug)]
pub struct ArrayInfo {
    name: String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,

    element: Type,
    length: usize,
}

impl CommonTypeInfo for ArrayInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.assoc_consts)()
    }
}

#[derive(Debug)]
pub struct SliceInfo {
    name: String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,

    element: Type,
}

impl CommonTypeInfo for SliceInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.assoc_consts)()
    }
}

#[derive(Debug)]
pub struct PointerInfo {
    name: String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,

    element: Type,
    mutability: bool,
}

impl CommonTypeInfo for PointerInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.assoc_consts)()
    }
}

#[derive(Debug)]
pub struct ReferenceInfo {
    name: String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,

    element: Type,
    mutability: bool,
}

impl CommonTypeInfo for ReferenceInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.assoc_consts)()
    }
}

#[derive(Debug)]
pub struct StructInfo {
    name: String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,

    fields: fn() -> Vec<NamedField>,
}

impl StructInfo {
    pub fn fields(&self) -> Vec<NamedField> {
        (self.fields)()
    }
}

impl CommonTypeInfo for StructInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.assoc_consts)()
    }
}

#[derive(Debug)]
pub struct EnumInfo {
    name: String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,
    
    variants: fn() -> Vec<VariantInfo>,
}

impl CommonTypeInfo for EnumInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.assoc_consts)()
    }
}

#[derive(Debug)]
pub struct TupleStructInfo {
    name: String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,

    fields: fn() -> Vec<TupleField>,
}

impl CommonTypeInfo for TupleStructInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.assoc_consts)()
    }
}

#[derive(Debug)]
pub struct UnitStructInfo {
    name: String,
    assoc_fns: fn() -> Vec<AssocFn>,
    assoc_consts: fn() -> Vec<AssocConst>,
}

impl CommonTypeInfo for UnitStructInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn assoc_fns(&self) -> Vec<AssocFn> {
        (self.assoc_fns)()
    }

    fn assoc_consts(&self) -> Vec<AssocConst> {
        (self.assoc_consts)()
    }
}

