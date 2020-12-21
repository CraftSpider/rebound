use crate::{Error, Type, Value};

use std::fmt;

// TODO: AssocFn with non-'static return types?
pub(crate) type CallFnHelper = Box<dyn Fn(Option<Value>, Vec<Value>) -> Value<'static>>;

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
            Some(ty) => match &this {
                Some(val) => {
                    if val.ty() != ty {
                        return Err(Error::wrong_type(val.ty(), ty));
                    }
                }
                None => return Err(Error::ExpectedSelf),
            },
            None => {
                if this.is_some() {
                    return Err(Error::UnexpectedSelf);
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
                return Err(Error::wrong_type(val.ty(), self.args[idx]));
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
