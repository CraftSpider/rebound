//! Runtime information about a Trait

#![allow(dead_code, unused)]

use crate::info::*;

use std::collections::BTreeMap;
use std::sync::RwLock;
use linkme::distributed_slice;

#[distributed_slice]
pub static REBOUND_TRAITS: [fn() -> Trait] = [..];

#[derive(Debug)]
pub struct Trait {
    name: &'static str,
    bounds: fn() -> &'static [Trait],
    methods: fn() -> &'static [/*TraitFn*/ ()],
}

impl Trait {
    #[doc(hidden)]
    pub fn new_trait(
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

// Traits can be reflected by generating a function alongside them with the same name, which
// returns the necessary info for reflection. A macro is dangerous as it's much more common to have
// a macro with the same name as a trait, as that's the derive convention.
// This can break down if the user also wants a function with that name, but it's the best we have
// currently.

#[distributed_slice(REBOUND_TRAITS)]
fn _clone() -> Trait {
    Trait::new_trait(
        "Clone",
        || &[],
        || &[],
    )
}

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

// trait Bar<T> {
//     fn foo() -> T {
//         !
//     }
// }
