use super::{AccessHelper, SetHelper};
use crate::{Error, Type, Value};

pub struct UnionField {
    get_ptr: Option<AccessHelper>,
    set_ptr: Option<SetHelper>,
    name: &'static str,
    assoc_ty: Type,
    field_ty: Type,
}

impl UnionField {
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

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    pub fn ty(&self) -> Type {
        self.field_ty
    }

    pub unsafe fn get_ref<'a>(&self, this: &'a Value<'a>) -> Result<Value<'a>, Error> {
        if let Some(get_ptr) = &self.get_ptr {
            if this.ty() != self.assoc_ty() {
                Err(Error::wrong_type(this.ty(), self.assoc_ty))
            } else {
                Ok((get_ptr)(this))
            }
        } else {
            Err(Error::UnsupportedOperation)
        }
    }

    pub unsafe fn set(&self, this: &mut Value, other: Value<'static>) -> Result<(), Error> {
        if let Some(set_ptr) = &self.set_ptr {
            if this.ty() != self.assoc_ty() || other.ty() != self.ty() {
                Err(Error::wrong_type(this.ty(), self.assoc_ty))
            } else {
                (set_ptr)(this, other);
                Ok(())
            }
        } else {
            Err(Error::UnsupportedOperation)
        }
    }
}
