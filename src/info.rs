//! Implementations of the various items that Types may possess. These items tend to support
//! the kind of operations that can be performed on that item during compile time, such as
//! calling a function or retrieving the value of a field.

use crate::Value;

mod consts;
mod fields;
mod fns;
mod union_fields;
mod variants;

pub use consts::AssocConst;
pub use fields::{Field, FieldKind};
pub use fns::AssocFn;
pub use union_fields::UnionField;
pub use variants::{StructVariant, TupleVariant, UnitVariant, Variant};

type AccessHelper = Box<dyn for<'a> Fn(&'a Value<'a>) -> Value<'a>>;
type SetHelper = Box<dyn Fn(&mut Value, Value<'static>)>;
