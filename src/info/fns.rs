use crate::{Error, Type, Value};

use std::fmt;

// TODO: AssocFn with non-'static return types?
pub(crate) type CallStaticHelper = Box<dyn Fn(Vec<Value>) -> Value<'static>>;
pub(crate) type CallDynamicHelper = Box<dyn Fn(Value, Vec<Value>) -> Value<'static>>;

pub enum FnKind {
    Static {
        call: CallStaticHelper,
    },
    Dynamic {
        call: CallDynamicHelper,
        self_ty: Type,
    },
}

impl fmt::Debug for FnKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FnKind::Static { call } => write!(f, "Static {{ call: {:p} }}", call),
            FnKind::Dynamic { call, self_ty } => write!(f, "Dynamic {{ call: {:p}, self_ty: {:?} }}", call, self_ty)
        }
    }
}

#[derive(Debug)]
pub struct AssocFn {
    name: &'static str,
    assoc_ty: Type,
    args: Vec<Type>,
    ret: Type,
    kind: FnKind,
}

impl AssocFn {
    pub unsafe fn new_static(
        call: CallStaticHelper,
        name: &'static str,
        assoc_ty: Type,
        args: &[Type],
        ret: Type,
    ) -> AssocFn {
        AssocFn {
            name,
            assoc_ty,
            args: args.to_owned(),
            ret,
            kind: FnKind::Static { call },
        }
    }

    pub unsafe fn new_dynamic(
        call: CallDynamicHelper,
        name: &'static str,
        assoc_ty: Type,
        self_ty: Type,
        args: &[Type],
        ret: Type,
    ) -> AssocFn {
        AssocFn {
            name,
            assoc_ty,
            args: args.to_owned(),
            ret,
            kind: FnKind::Dynamic { call, self_ty },
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

    pub fn kind(&self) -> &FnKind {
        &self.kind
    }

    pub fn call(&self, this: Option<Value>, args: Vec<Value>) -> Result<Value<'static>, Error> {
        // Check the validity of `this`
        match self.kind {
            FnKind::Dynamic { self_ty, .. } => match &this {
                Some(val) => {
                    if val.ty() != self_ty {
                        return Err(Error::wrong_type(val.ty(), self_ty));
                    }
                }
                None => return Err(Error::IsDynamic),
            },
            FnKind::Static {..} => {
                if this.is_some() {
                    return Err(Error::IsStatic);
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
        Ok(match &self.kind {
            FnKind::Static { call } => call(args),
            FnKind::Dynamic { call, .. } => call(this.unwrap(), args)
        })
    }
}
