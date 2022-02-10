//! Runtime information about a Trait

#![allow(dead_code, unused)]

use crate::info::*;

use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::RwLock;

static REFLECTED_TRAITS: OnceCell<RwLock<HashMap<String, Box<Trait>>>> = OnceCell::new();

#[derive(Debug)]
struct Trait {
    name: String,
    bounds: fn() -> Vec<Trait>,
    methods: fn() -> Vec</*TraitFn*/ ()>,
}

impl Trait {
    fn add_trait(tr: Trait) {
        let mut map = REFLECTED_TRAITS
            .get_or_init(|| RwLock::new(HashMap::new()))
            .write()
            .expect("REFLECTED_TRAITS not initialized correctly");

        let name = tr.name();

        if map.contains_key(name) {
            panic!("Trait {} already registered", name);
        }

        map.insert(name.clone(), Box::new(tr));
    }

    pub unsafe fn new_trait(
        name: String,
        bounds: fn() -> Vec<Trait>,
        methods: fn() -> Vec</*TraitFn*/ ()>,
    ) -> Trait {
        Trait {
            name,
            bounds,
            methods,
        }
        // Trait::add_trait(tr);
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn bounds(&self) -> Vec<Trait> {
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
