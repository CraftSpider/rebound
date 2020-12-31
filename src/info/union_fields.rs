use super::{AccessHelper, SetHelper};
use crate::{Error, Type, Value};

/// Info about a field on a union [`Type`]. Separate from a normal [`Field`](crate::Field) due to
/// the unsafety of accessing/setting union fields. Allows getting a reference to or setting the
/// content of this field on a [`Value`], assuming the reflection was configured to allow it.
pub struct UnionField {
    get_ptr: Option<AccessHelper>,
    set_ptr: Option<SetHelper>,
    name: &'static str,
    assoc_ty: Type,
    field_ty: Type,
}

impl UnionField {
    /// Internal Function, used to create a new union field
    ///
    /// # Safety
    ///
    /// Should only be called within a `ReflectedUnion`'s `fields` implementation
    pub unsafe fn new(
        get_ptr: Option<AccessHelper>,
        set_ptr: Option<SetHelper>,
        name: &'static str,
        assoc_ty: Type,
        field_ty: Type,
    ) -> UnionField {
        UnionField {
            get_ptr,
            set_ptr,
            name,
            assoc_ty,
            field_ty,
        }
    }

    /// Get the name of this field
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the Type this field is defined on
    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    /// Get the Type of the data in this field
    pub fn ty(&self) -> Type {
        self.field_ty
    }

    /// Get a reference to the data contained within this field on a [`Value`], assuming the Value
    /// is of the correct type and the operation is supported.
    ///
    /// # Safety
    ///
    /// This is unsafe for the same reason accessing a field on a union normally is, and assumes
    /// that the union content can be correctly interpreted as a value of this type.
    pub unsafe fn get_ref<'a>(&self, this: &'a Value<'a>) -> Result<Value<'a>, Error> {
        self.get_ptr
            .as_ref()
            .map_or(Err(Error::UnsupportedOperation), |get_ptr| {
                if this.ty() == self.assoc_ty() {
                    Ok((get_ptr)(this))
                } else {
                    Err(Error::wrong_type(this.ty(), self.assoc_ty))
                }
            })
    }

    /// Set the data contained within this Field on a [`Value`], assuming the Values of both `this`
    /// and `other` are of the correct type and the operation is supported.
    ///
    /// # Safety
    ///
    /// This is unsafe for the same reason setting a non-Copy field on a union normally is,
    /// reading the field may require the union content be read as a value of the type being set.
    #[allow(clippy::suspicious_operation_groupings)]
    pub unsafe fn set(&self, this: &mut Value, other: Value<'static>) -> Result<(), Error> {
        self.set_ptr
            .as_ref()
            .map_or(Err(Error::UnsupportedOperation), |set_ptr| {
                if this.ty() == self.assoc_ty() && other.ty() == self.ty() {
                    (set_ptr)(this, other);
                    Ok(())
                } else {
                    Err(Error::wrong_type(this.ty(), self.assoc_ty))
                }
            })
    }
}
