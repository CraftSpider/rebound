use crate::{Error, Type, Value, Variant};

use std::fmt;

pub(crate) type AccessHelper = Box<dyn for<'a> Fn(&'a Value<'a>) -> Value<'a>>;
pub(crate) type SetHelper = Box<dyn Fn(&mut Value, Value<'static>)>;

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

// TODO: Optional get/set, make macro support #[rebound(no_get, no_set)]

pub struct Field {
    get_ptr: AccessHelper,
    set_ptr: SetHelper,
    assoc_ty: Type,
    field_ty: Type,
    kind: FieldKind,
}

impl Field {
    pub unsafe fn new_named(
        get_ptr: AccessHelper,
        set_ptr: SetHelper,
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
        get_ptr: AccessHelper,
        set_ptr: SetHelper,
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
        get_ptr: AccessHelper,
        set_ptr: SetHelper,
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
        get_ptr: AccessHelper,
        set_ptr: SetHelper,
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
        if this.ty() != self.assoc_ty() {
            Err(Error::wrong_type(this.ty(), self.assoc_ty))
        } else {
            Ok((self.get_ptr)(this))
        }
    }

    pub fn set(&self, this: &mut Value, other: Value<'static>) -> Result<(), Error> {
        if this.ty() != self.assoc_ty() || other.ty() != self.ty() {
            Err(Error::wrong_type(this.ty(), self.assoc_ty))
        } else {
            (self.set_ptr)(this, other);
            Ok(())
        }
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Field {{ get_ptr: {:p}, set_ptr: {:p}, assoc_ty: {:?}, field_ty: {:?}, kind: {:?} }}",
            self.get_ptr, self.set_ptr, self.assoc_ty, self.field_ty, self.kind
        )
    }
}
