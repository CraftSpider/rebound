//! Implementations of the various items that Types may possess. These items tend to support
//! the kind of operations that can be performed on that item during compile time, such as
//! calling a function or retrieving the value of a field.

use crate::reflect::RefHack;
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

#[doc(hidden)]
pub type AccessHelper = for<'a, 'b> fn(&'a Value<'b>) -> Value<'a>;
#[doc(hidden)]
pub type SetHelper = for<'a, 'b> fn(RefHack<'a, 'b>, Value<'static>);
