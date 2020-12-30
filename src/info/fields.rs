use crate::{Error, Type, Value, Variant};
use super::{AccessHelper, SetHelper};

use core::fmt;

#[derive(Debug)]
pub enum FieldKind {
    Tuple {
        idx: usize,
    },
    Named {
        name: &'static str,
    },
    EnumTuple {
        idx: usize,
        assoc_var: Variant,
    },
    EnumNamed {
        name: &'static str,
        assoc_var: Variant,
    },
}

pub struct Field {
    get_ptr: Option<AccessHelper>,
    set_ptr: Option<SetHelper>,
    assoc_ty: Type,
    field_ty: Type,
    kind: FieldKind,
}

impl Field {
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

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    pub fn ty(&self) -> Type {
        self.field_ty
    }

    pub fn kind(&self) -> &FieldKind {
        &self.kind
    }

    pub fn get_ref<'a>(&self, this: &'a Value<'a>) -> Result<Value<'a>, Error> {
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

    pub fn set(&self, this: &mut Value, other: Value<'static>) -> Result<(), Error> {
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

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Field {{ get_ptr: _, set_ptr: _, assoc_ty: {:?}, field_ty: {:?}, kind: {:?} }}",
            self.assoc_ty, self.field_ty, self.kind
        )
    }
}
