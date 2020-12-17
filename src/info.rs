// #[allow(unused_imports)]
// use crate::prelude::*;

use crate::{Error, Type, Value};

use core::fmt;

pub(crate) type CallFnHelper = Box<dyn Fn(Option<Value>, Vec<Value>) -> Value<'static>>;
pub(crate) type GetConstHelper = Box<dyn Fn() -> Value<'static>>;

pub(crate) type AccessHelper = Box<dyn for<'a> Fn(&'a Value<'a>) -> Value<'a>>;
pub(crate) type SetHelper = Box<dyn Fn(&mut Value, Value<'static>)>;

pub struct AssocFn {
    ptr: CallFnHelper,
    name: &'static str,
    assoc_ty: Type,
    self_ty: Option<Type>,
    args: Vec<Type>,
    ret: Type,
}

impl AssocFn {
    pub unsafe fn new(
        ptr: CallFnHelper,
        name: &'static str,
        assoc_ty: Type,
        self_ty: Option<Type>,
        args: &[Type],
        ret: Type,
    ) -> AssocFn {
        AssocFn {
            ptr,
            name,
            assoc_ty,
            self_ty,
            args: Vec::from(args),
            ret,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    pub fn arg_tys(&self) -> &Vec<Type> {
        &self.args
    }

    pub fn ret_ty(&self) -> Type {
        self.ret
    }

    pub fn call(&self, this: Option<Value>, args: Vec<Value>) -> Result<Value<'static>, Error> {
        // Check the validity of `this`
        match self.self_ty {
            Some(ty) => {
                match &this {
                    Some(val) => if val.ty() != ty {
                        return Err(Error::WrongType)
                    }
                    None => {
                        return Err(Error::ExpectedSelf)
                    }
                }
            }
            None => {
                if this.is_some() {
                    return Err(Error::UnexpectedSelf)
                }
            }
        }

        // Check the validity of `args`
        if args.len() > self.args.len() {
            return Err(Error::TooManyArgs);
        } else if args.len() < self.args.len() {
            return Err(Error::TooFewArgs);
        }

        for (idx, val) in args.iter().enumerate() {
            if val.ty() != self.args[idx] {
                return Err(Error::WrongType)
            }
        }

        // Actually call the helper func
        Ok((self.ptr)(this, args))
    }
}

impl fmt::Debug for AssocFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AssocFn {{ ptr: {:p}, name: {:?}, assoc_ty: {:?}, self_ty: {:?}, args: {:?}, ret: {:?} }}",
            self.ptr, self.name, self.assoc_ty, self.self_ty, self.args, self.ret
        )
    }
}

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

pub struct TupleField {
    get_ptr: AccessHelper,
    set_ptr: SetHelper,
    pos: usize,
    assoc_ty: Type,
    ty: Type,
}

impl TupleField {
    pub unsafe fn new(
        get_ptr: AccessHelper,
        set_ptr: SetHelper,
        pos: usize,
        assoc_ty: Type,
        ty: Type,
    ) -> TupleField {
        TupleField {
            get_ptr,
            set_ptr,
            pos,
            assoc_ty,
            ty,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    pub fn ty(&self) -> Type {
        self.ty
    }

    pub fn get_ref<'a>(&self, this: &'a Value<'a>) -> Result<Value<'a>, Error> {
        if this.ty() != self.assoc_ty() {
            Err(Error::WrongType)
        } else {
            Ok((self.get_ptr)(this))
        }
    }

    pub fn set(&self, this: &mut Value, other: Value<'static>) -> Result<(), Error> {
        if this.ty() != self.assoc_ty() || other.ty() != self.ty() {
            Err(Error::WrongType)
        } else {
            (self.set_ptr)(this, other);
            Ok(())
        }
    }
}

impl fmt::Debug for TupleField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TupleField {{ get_ptr: {:p}, set_ptr: {:p}, pos: {:?}, assoc_ty: {:?}, ty: {:?} }}",
            self.get_ptr, self.set_ptr, self.pos, self.assoc_ty, self.ty
        )
    }
}

pub struct NamedField {
    get_ptr: AccessHelper,
    set_ptr: SetHelper,
    name: &'static str,
    assoc_ty: Type,
    ty: Type,
}

impl NamedField {
    pub unsafe fn new(
        get_ptr: AccessHelper,
        set_ptr: SetHelper,
        name: &'static str,
        assoc_ty: Type,
        ty: Type,
    ) -> NamedField {
        NamedField {
            get_ptr,
            set_ptr,
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

    pub fn get_ref<'a>(&self, this: &'a Value<'a>) -> Result<Value<'a>, Error> {
        if this.ty() != self.assoc_ty() {
            Err(Error::WrongType)
        } else {
            Ok((self.get_ptr)(this))
        }
    }

    pub fn set(&self, this: &mut Value, other: Value<'static>) -> Result<(), Error> {
        if this.ty() != self.assoc_ty() || other.ty() != self.ty() {
            Err(Error::WrongType)
        } else {
            (self.set_ptr)(this, other);
            Ok(())
        }
    }
}

impl fmt::Debug for NamedField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NamedField {{ set_ptr: {:p}, get_ptr: {:p}, name: {:?}, assoc_ty: {:?}, ty: {:?} }}",
            self.set_ptr, self.get_ptr, self.name, self.assoc_ty, self.ty
        )
    }
}

#[derive(Debug)]
pub enum VariantInfo {
    Empty(EmptyVariant),
    Tuple(TupleVariant),
    StructVariant(StructVariant)
}

#[derive(Debug)]
pub struct EmptyVariant {
    name: &'static str
}

#[derive(Debug)]
pub struct TupleVariant {
    name: &'static str,
    fields: fn() -> Vec<TupleField>
}

#[derive(Debug)]
pub struct StructVariant {
    name: &'static str,
    fields: fn() -> Vec<NamedField>
}
