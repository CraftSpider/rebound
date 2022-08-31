use crate::{Error, Type, Value};

use core::fmt;

type CallStaticHelper = for<'a> fn(Vec<Value<'a>>) -> Value<'a>;
type CallDynamicHelper = for<'a> fn(Value<'a>, Vec<Value<'a>>) -> Value<'a>;

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
            FnKind::Dynamic { call, self_ty } => {
                write!(f, "Dynamic {{ call: {:p}, self_ty: {:?} }}", call, self_ty)
            }
        }
    }
}

/// Info about an associated function on a [`Type`], either dynamic or static. Allows calling the
/// function, assuming reflection was configured to allow it.
#[derive(Debug)]
pub struct AssocFn {
    name: &'static str,
    assoc_ty: Type,
    args: Vec<Type>,
    ret: Type,
    kind: FnKind,
}

impl AssocFn {
    /// Internal Function, creates a new static function reference
    ///
    /// # Safety
    ///
    /// Should only be called within a `ReflectedImpl`'s `assoc_fns` implementation
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

    /// Internal Function, creates a new dynamic function reference
    ///
    /// # Safety
    ///
    /// Should only be called within a `ReflectedImpl`'s `assoc_fns` implementation
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

    /// Get the name of this function in code
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the Type this function was defined on
    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    /// Get the Types of the arguments to this function
    pub fn arg_tys(&self) -> &[Type] {
        &self.args
    }

    /// Get the return Type of this function
    pub fn ret_ty(&self) -> Type {
        self.ret
    }

    /// Get the kind of this function, which contains specific information
    pub fn kind(&self) -> &FnKind {
        &self.kind
    }

    /// Attempt to call this function with the provided receiver and arguments.
    ///
    /// The receiver should only be provided if this is a dynamic function, the number of
    /// arguments must exactly match the number required to call the function, and reflection
    /// must be configured to support calling this function.
    pub fn call<'a>(
        &self,
        this: Option<Value<'a>>,
        args: Vec<Value<'a>>,
    ) -> Result<Value<'a>, Error> {
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
            FnKind::Static { .. } => {
                if this.is_some() {
                    return Err(Error::IsStatic);
                }
            }
        }

        // Check the validity of `args`
        if args.len() != self.args.len() {
            return Err(Error::wrong_args_num(args.len(), self.args.len()));
        }

        for (idx, val) in args.iter().enumerate() {
            if val.ty() != self.args[idx] {
                return Err(Error::wrong_type(val.ty(), self.args[idx]));
            }
        }

        // Actually call the helper func
        Ok(match (&self.kind, this) {
            (FnKind::Static { call }, None) => call(args),
            (FnKind::Dynamic { call, .. }, Some(this)) => call(this, args),
            _ => unreachable!(),
        })
    }
}
