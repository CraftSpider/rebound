use crate::{Type, Value};

use std::fmt;

pub(crate) type GetConstHelper = Box<dyn Fn() -> Value<'static>>;

pub struct AssocConst {
    ptr: GetConstHelper,
    name: &'static str,
    assoc_ty: Type,
    ty: Type,
}

impl AssocConst {
    pub unsafe fn new(
        ptr: GetConstHelper,
        name: &'static str,
        assoc_ty: Type,
        ty: Type,
    ) -> AssocConst {
        AssocConst {
            ptr,
            name,
            assoc_ty,
            ty,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    pub fn ty(&self) -> Type {
        self.ty
    }

    pub fn get(&self) -> Value<'static> {
        (self.ptr)()
    }
}

impl fmt::Debug for AssocConst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AssocConst {{ ptr: {:p}, name: {:?}, assoc_ty: {:?}, ty: {:?} }}",
            self.ptr, self.name, self.assoc_ty, self.ty
        )
    }
}
