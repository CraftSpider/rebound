use crate::{Error, Type, Value};

use core::fmt;

type GetConstHelper = Box<dyn Fn() -> Value<'static>>;

/// Info about a constant on a [`Type`]. Allows getting the Value of this constant, assuming the
/// reflection was configured to allow it.
pub struct AssocConst {
    ptr: GetConstHelper,
    name: &'static str,
    assoc_ty: Type,
    ty: Type,
}

impl AssocConst {
    /// Internal Function, creates a new constant
    ///
    /// # Safety
    ///
    /// Should only be called within a `ReflectedImpl`'s `assoc_consts` implementation
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

    /// Get the name of this constant in code
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the Type this constant was defined on
    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    /// Get the Type of the data in this constant
    pub fn ty(&self) -> Type {
        self.ty
    }

    /// Get the value of this constant, if the operation is supported.
    pub fn get(&self) -> Result<Value<'static>, Error> {
        Ok((self.ptr)())
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
