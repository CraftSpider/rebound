use crate::Value;

mod consts;
mod fields;
mod fns;
mod variants;
mod union_fields;

pub use consts::AssocConst;
pub use fields::{Field, FieldKind};
pub use fns::AssocFn;
pub use variants::{StructVariant, TupleVariant, UnitVariant, Variant};
pub use union_fields::UnionField;

pub(crate) type AccessHelper = Box<dyn for<'a> Fn(&'a Value<'a>) -> Value<'a>>;
pub(crate) type SetHelper = Box<dyn Fn(&mut Value, Value<'static>)>;
