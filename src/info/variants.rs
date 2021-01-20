use crate::{Error, Field, Type, Value};

use core::fmt;

type IsVarHelper = fn(&Value) -> bool;

/// Info about a variant on an enum [`Type`]. Allows accessing the name and fields of the given
/// variant.
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    /// A unit variant containing no fields
    Unit(UnitVariant),
    /// A tuple variant, containing unnamed fields
    Tuple(TupleVariant),
    /// A struct variant, containing named fields
    Struct(StructVariant),
}

impl Variant {
    /// Get the name of the current variant
    pub fn name(&self) -> &str {
        match self {
            Variant::Unit(var) => var.name(),
            Variant::Tuple(var) => var.name(),
            Variant::Struct(var) => var.name(),
        }
    }

    /// Get the Type this variant is defined on
    pub fn assoc_ty(&self) -> Type {
        match self {
            Variant::Unit(var) => var.assoc_ty(),
            Variant::Tuple(var) => var.assoc_ty(),
            Variant::Struct(var) => var.assoc_ty(),
        }
    }

    /// Check whether a given [`Value`] is this variant
    pub fn is_variant(&self, val: &Value) -> Result<bool, Error> {
        match self {
            Variant::Unit(var) => var.is_variant(val),
            Variant::Tuple(var) => var.is_variant(val),
            Variant::Struct(var) => var.is_variant(val),
        }
    }
}

/// Info specific to a Unit variant
#[derive(Clone)]
pub struct UnitVariant {
    name: &'static str,
    assoc_ty: Type,
    is_var: IsVarHelper,
}

impl UnitVariant {
    /// Internal Function, used to create a new unit variant.
    ///
    /// # Safety
    ///
    /// Should only be called within a `ReflectedEnum`'s `variants` implementation
    pub unsafe fn new(name: &'static str, assoc_ty: Type, is_var: IsVarHelper) -> UnitVariant {
        UnitVariant {
            name,
            assoc_ty,
            is_var,
        }
    }

    /// Get the name of this variant
    pub fn name(&self) -> &str {
        self.name
    }

    /// Get the Type this variant is defined on
    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    /// Check whether a given [`Value`] is this variant
    pub fn is_variant(&self, val: &Value) -> Result<bool, Error> {
        if val.ty() == self.assoc_ty() {
            Ok((self.is_var)(val))
        } else {
            Err(Error::wrong_type(val.ty(), self.assoc_ty()))
        }
    }
}

impl PartialEq for UnitVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.assoc_ty == other.assoc_ty
    }
}

impl fmt::Debug for UnitVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UnitVariant {{ name: {:?}, assoc_ty: {:?}, is_var: {:p} }}",
            self.name, self.assoc_ty, self.is_var as *const ()
        )
    }
}

/// Info specific to a Tuple variant
#[derive(Clone)]
pub struct TupleVariant {
    name: &'static str,
    assoc_ty: Type,
    fields: fn() -> Vec<Field>,
    is_var: IsVarHelper,
}

impl TupleVariant {
    /// Internal Function, used to create a new tuple variant.
    ///
    /// # Safety
    ///
    /// Should only be called within a `ReflectedEnum`'s `variants` implementation
    pub unsafe fn new(
        name: &'static str,
        assoc_ty: Type,
        fields: fn() -> Vec<Field>,
        is_var: IsVarHelper,
    ) -> TupleVariant {
        TupleVariant {
            name,
            assoc_ty,
            fields,
            is_var,
        }
    }

    /// Get the name of this variant
    pub fn name(&self) -> &str {
        self.name
    }

    /// Get the Type this variant is defined on
    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    /// Get the unnamed fields of this variant
    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }

    /// Check whether a given [`Value`] is this variant
    pub fn is_variant(&self, val: &Value) -> Result<bool, Error> {
        if val.ty() == self.assoc_ty() {
            Ok((self.is_var)(val))
        } else {
            Err(Error::wrong_type(val.ty(), self.assoc_ty()))
        }
    }
}

impl PartialEq for TupleVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.assoc_ty == other.assoc_ty
    }
}

impl fmt::Debug for TupleVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TupleVariant {{ name: {:?}, assoc_ty: {:?}, fields: {:?}, is_var: {:p} }}",
            self.name, self.assoc_ty, self.fields, self.is_var as *const ()
        )
    }
}

/// Info specific to a Struct variant
#[derive(Clone)]
pub struct StructVariant {
    name: &'static str,
    assoc_ty: Type,
    fields: fn() -> Vec<Field>,
    is_var: IsVarHelper,
}

impl StructVariant {
    /// Internal Function, used to create a new struct variant.
    ///
    /// # Safety
    ///
    /// Should only be called within a `ReflectedEnum`'s `variants` implementation
    pub unsafe fn new(
        name: &'static str,
        assoc_ty: Type,
        fields: fn() -> Vec<Field>,
        is_var: IsVarHelper,
    ) -> StructVariant {
        StructVariant {
            name,
            assoc_ty,
            fields,
            is_var,
        }
    }

    /// Get the name of this variant
    pub fn name(&self) -> &str {
        self.name
    }

    /// Get the Type this variant is defined on
    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    /// Get the named fields of this variant
    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }

    /// Check whether a given [`Value`] is this variant
    pub fn is_variant(&self, val: &Value) -> Result<bool, Error> {
        if val.ty() == self.assoc_ty() {
            Ok((self.is_var)(val))
        } else {
            Err(Error::wrong_type(val.ty(), self.assoc_ty()))
        }
    }
}

impl PartialEq for StructVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.assoc_ty == other.assoc_ty
    }
}

impl fmt::Debug for StructVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StructVariant {{ name: {:?}, assoc_ty: {:?}, fields: {:?}, is_var: {:p} }}",
            self.name, self.assoc_ty, self.fields, self.is_var as *const ()
        )
    }
}
