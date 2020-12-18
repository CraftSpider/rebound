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
        assoc_var: VariantInfo,
    },
    EnumNamed {
        name: &'static str,
        assoc_var: VariantInfo,
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
        assoc_var: VariantInfo,
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
        assoc_var: VariantInfo,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VariantInfo {
    Unit(UnitVariant),
    Tuple(TupleVariant),
    Struct(StructVariant),
}

impl VariantInfo {
    pub fn name(&self) -> &str {
        match self {
            VariantInfo::Unit(var) => var.name(),
            VariantInfo::Tuple(var) => var.name(),
            VariantInfo::Struct(var) => var.name(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UnitVariant {
    name: &'static str,
    assoc_ty: Type,
}

impl UnitVariant {
    pub unsafe fn new(name: &'static str, assoc_ty: Type) -> UnitVariant {
        UnitVariant { name, assoc_ty }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }
}

impl PartialEq for UnitVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.assoc_ty == other.assoc_ty
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TupleVariant {
    name: &'static str,
    assoc_ty: Type,
    fields: fn() -> Vec<Field>,
}

impl TupleVariant {
    pub unsafe fn new(
        name: &'static str,
        assoc_ty: Type,
        fields: fn() -> Vec<Field>,
    ) -> TupleVariant {
        TupleVariant {
            name,
            assoc_ty,
            fields,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }
}

impl PartialEq for TupleVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.assoc_ty == other.assoc_ty
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StructVariant {
    name: &'static str,
    assoc_ty: Type,
    fields: fn() -> Vec<Field>,
}

impl StructVariant {
    pub unsafe fn new(
        name: &'static str,
        assoc_ty: Type,
        fields: fn() -> Vec<Field>,
    ) -> StructVariant {
        StructVariant {
            name,
            assoc_ty,
            fields,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn assoc_ty(&self) -> Type {
        self.assoc_ty
    }

    pub fn fields(&self) -> Vec<Field> {
        (self.fields)()
    }
}

impl PartialEq for StructVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.assoc_ty == other.assoc_ty
    }
}
