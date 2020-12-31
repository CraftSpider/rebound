use super::{AccessHelper, SetHelper};
use crate::{Error, Type, Value, Variant};

use core::fmt;

/// Info about different kinds of [`Fields`](Field)
#[derive(Debug)]
pub enum FieldKind {
    /// A tuple field, accessed by numeric index
    Tuple {
        /// The index of this field
        idx: usize,
    },
    /// A named field, accessed by textual name
    Named {
        /// The name of this field
        name: &'static str,
    },
    /// An enum tuple field, accessed by variant and numeric index
    EnumTuple {
        /// The index of this field
        idx: usize,
        /// The variant of this field
        assoc_var: Variant,
    },
    /// An enum named field, accessed by variant and textual name
    EnumNamed {
        /// The name of this field
        name: &'static str,
        /// The variant of this field
        assoc_var: Variant,
    },
}

/// Info about a field on a [`Type`], either named or unnamed. Allows getting a reference to or
/// setting the content of this field on a [`Value`], assuming the reflection was configured to
/// allow it.
pub struct Field {
    get_ptr: Option<AccessHelper>,
    set_ptr: Option<SetHelper>,
    assoc_ty: Type,
    field_ty: Type,
    kind: FieldKind,
}

impl Field {
    /// Internal Function, creates a new named field
    ///
    /// # Safety
    ///
    /// Should only be called within a Reflected item's `fields` implementation
    pub unsafe fn new_named(
        get_ptr: Option<AccessHelper>,
        set_ptr: Option<SetHelper>,
        name: &'static str,
        assoc_ty: Type,
        field_ty: Type,
    ) -> Field {
        Field {
            get_ptr,
            set_ptr,
            assoc_ty,
            field_ty,
            kind: FieldKind::Named { name },
        }
    }

    /// Internal Function, creates a new unnamed field
    ///
    /// # Safety
    ///f
    /// Should only be called within a Reflected item's `fields` implementation
    pub unsafe fn new_tuple(
        get_ptr: Option<AccessHelper>,
        set_ptr: Option<SetHelper>,
        idx: usize,
        assoc_ty: Type,
        field_ty: Type,
    ) -> Field {
        Field {
            get_ptr,
            set_ptr,
            assoc_ty,
            field_ty,
            kind: FieldKind::Tuple { idx },
        }
    }

    /// Internal Function, creates a new named field for a variant
    ///
    /// # Safety
    ///
    /// Should only be called within a Reflected item's `fields` implementation
    pub unsafe fn new_enum_named(
        get_ptr: Option<AccessHelper>,
        set_ptr: Option<SetHelper>,
        name: &'static str,
        assoc_ty: Type,
        assoc_var: Variant,
        field_ty: Type,
    ) -> Field {
        Field {
            get_ptr,
            set_ptr,
            assoc_ty,
            field_ty,
            kind: FieldKind::EnumNamed { name, assoc_var },
        }
    }

    /// Internal Function, creates a new unnamed field for a variant
    ///
    /// # Safety
    ///
    /// Should only be called within a Reflected item's `fields` implementation
    pub unsafe fn new_enum_tuple(
        get_ptr: Option<AccessHelper>,
        set_ptr: Option<SetHelper>,
        idx: usize,
        assoc_ty: Type,
        assoc_var: Variant,
        field_ty: Type,
    ) -> Field {
        Field {
            get_ptr,
            set_ptr,
            assoc_ty,
            field_ty,
            kind: FieldKind::EnumTuple { idx, assoc_var },
        }
    }

    /// Get the Type this field is defined on
    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    /// Get the Type of the data in this field
    pub fn ty(&self) -> Type {
        self.field_ty
    }

    /// Get the kind of this field, which contains name or index information
    pub fn kind(&self) -> &FieldKind {
        &self.kind
    }

    /// Get a reference to the data contained within this Field on a [`Value`], assuming the Value
    /// is of the correct type and the operation is supported.
    pub fn get_ref<'a>(&self, this: &'a Value<'a>) -> Result<Value<'a>, Error> {
        self.get_ptr
            .as_ref()
            .map_or(Err(Error::UnsupportedOperation), |get_ptr| {
                if this.ty() == self.assoc_ty() {
                    if let FieldKind::EnumTuple { assoc_var, .. }
                    | FieldKind::EnumNamed { assoc_var, .. } = self.kind()
                    {
                        if !assoc_var.is_variant(this).unwrap() {
                            let var = this.ty().unwrap_enum().variant_of(this).unwrap();
                            Err(Error::wrong_variant(&var, assoc_var))
                        } else {
                            Ok((get_ptr)(this))
                        }
                    } else {
                        Ok((get_ptr)(this))
                    }
                } else {
                    Err(Error::wrong_type(this.ty(), self.assoc_ty))
                }
            })
    }

    /// Set the data contained within this Field on a [`Value`], assuming the Values of both `this`
    /// and `other` are of the correct type and the operation is supported.
    #[allow(clippy::suspicious_operation_groupings)]
    pub fn set(&self, this: &mut Value, other: Value<'static>) -> Result<(), Error> {
        self.set_ptr
            .as_ref()
            .map_or(Err(Error::UnsupportedOperation), |set_ptr| {
                if this.ty() == self.assoc_ty() && other.ty() == self.ty() {
                    if let FieldKind::EnumTuple { assoc_var, .. }
                    | FieldKind::EnumNamed { assoc_var, .. } = self.kind()
                    {
                        if !assoc_var.is_variant(this).unwrap() {
                            let var = this.ty().unwrap_enum().variant_of(this).unwrap();
                            Err(Error::wrong_variant(&var, assoc_var))
                        } else {
                            (set_ptr)(this, other);
                            Ok(())
                        }
                    } else {
                        (set_ptr)(this, other);
                        Ok(())
                    }
                } else {
                    Err(Error::wrong_type(this.ty(), self.assoc_ty))
                }
            })
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Field {{ get_ptr: _, set_ptr: _, assoc_ty: {:?}, field_ty: {:?}, kind: {:?} }}",
            self.assoc_ty, self.field_ty, self.kind
        )
    }
}
