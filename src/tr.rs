use crate::info::*;

use std::collections::HashMap;
use std::sync::{Once, RwLock};

static INIT: Once = Once::new();
static mut REFLECTED_TRAITS: Option<RwLock<HashMap<String, Box<Trait>>>> = None;

#[derive(Debug)]
pub struct Trait {
    name: fn() -> String,
    bounds: fn() -> Vec<Trait>,
    methods: fn() -> Vec<AssocFn>,
}

impl Trait {
    fn ensure_statics() {
        INIT.call_once(|| {
            unsafe { REFLECTED_TRAITS = Some(RwLock::new(HashMap::new())) }
        })
    }

    fn add_trait(tr: Trait) {
        Trait::ensure_statics();

        let mut map = unsafe { REFLECTED_TRAITS.as_mut() }
            .expect("REFLECTED_TRAITS not initialized correctly")
            .write()
            .expect("REFLECTED_TRAITS not initialized correctly");

        let name = tr.name();

        if map.contains_key(&name) {
            panic!("Trait {} already registered", name);
        }

        map.insert(name, Box::new(tr));
    }

    pub unsafe fn new_trait(name: fn() -> String, bounds: fn() -> Vec<Trait>, methods: fn() -> Vec<AssocFn>) {
        let tr = Trait {
            name,
            bounds,
            methods
        };

        Trait::add_trait(tr);
    }

    pub fn name(&self) -> String {
        (self.name)()
    }

    pub fn bounds(&self) -> Vec<Trait> {
        (self.bounds)()
    }
}

//#[rebound]
trait Foo {
    fn meh(&self);
}

// TODO: How are traits reflected? This is needed to support dropping Values, as well as
//       things like formatting. Can't assume things will be `dyn`able
