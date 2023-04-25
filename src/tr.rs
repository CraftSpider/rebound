//! Runtime information about a Trait

#![allow(dead_code, unused)]

use crate::info::*;

use std::collections::BTreeMap;
use std::sync::RwLock;
use linkme::distributed_slice;

#[distributed_slice]
pub static REBOUND_TRAITS: [Trait] = [..];

#[derive(Debug)]
pub struct Trait {
    name: &'static str,
    bounds: fn() -> &'static [Trait],
    methods: fn() -> &'static [/*TraitFn*/ ()],
}

impl Trait {
    #[doc(hidden)]
    pub const fn new_trait(
        name: &'static str,
        bounds: fn() -> &'static [Trait],
        methods: fn() -> &'static [/*TraitFn*/ ()],
    ) -> Trait {
        Trait {
            name,
            bounds,
            methods,
        }
    }

    pub fn name(&self) -> &'static str {
        &self.name
    }

    pub fn bounds(&self) -> &'static [Trait] {
        (self.bounds)()
    }
}

// TODO: How are traits reflected? This is needed to support cloning Values, as well as
//       things like formatting. Can't assume things will be `dyn`able

// #[distributed_slice(REBOUND_TRAITS)]
// static CLONE: Trait = Trait::new_trait(
//     "Clone",
//     || &[],
//     || &[],
// );
//
// static ADD: Trait = Trait::new_trait(
//     "Add",
//     || &[],
//     || &[],
// );
//
// #[rebound]
// trait Foo: Sized {
//     type Assoc: Sized;
//
//     fn a();
//     fn b(&self);
//
//     fn c() {}
//     fn d(&self) {}
// }
//
// trait Bar<T> {
//     fn foo() -> T {
//         !
//     }
// }
