use crate::{Field, Type};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Variant {
    Unit(UnitVariant),
    Tuple(TupleVariant),
    Struct(StructVariant),
}

impl Variant {
    pub fn name(&self) -> &str {
        match self {
            Variant::Unit(var) => var.name(),
            Variant::Tuple(var) => var.name(),
            Variant::Struct(var) => var.name(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UnitVariant {
    name: &'static str,
    assoc_ty: Type,
}

impl UnitVariant {
    pub unsafe fn new(name: &'static str, assoc_ty: Type) -> UnitVariant {
        UnitVariant { name, assoc_ty }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }
}

impl PartialEq for UnitVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.assoc_ty == other.assoc_ty
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TupleVariant {
    name: &'static str,
    assoc_ty: Type,
    fields: fn() -> Vec<Field>,
}

impl TupleVariant {
    pub unsafe fn new(
        name: &'static str,
        assoc_ty: Type,
        fields: fn() -> Vec<Field>,
    ) -> TupleVariant {
        TupleVariant {
            name,
            assoc_ty,
            fields,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }
}

impl PartialEq for TupleVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.assoc_ty == other.assoc_ty
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StructVariant {
    name: &'static str,
    assoc_ty: Type,
    fields: fn() -> Vec<Field>,
}

impl StructVariant {
    pub unsafe fn new(
        name: &'static str,
        assoc_ty: Type,
        fields: fn() -> Vec<Field>,
    ) -> StructVariant {
        StructVariant {
            name,
            assoc_ty,
            fields,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }
}

impl PartialEq for StructVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.assoc_ty == other.assoc_ty
    }
}
