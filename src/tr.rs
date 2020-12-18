use crate::info::*;

use std::collections::HashMap;
use std::sync::RwLock;
use std::lazy::SyncOnceCell;

static REFLECTED_TRAITS: SyncOnceCell<RwLock<HashMap<String, Box<Trait>>>> = SyncOnceCell::new();

#[derive(Debug)]
pub struct Trait {
    name: fn() -> String,
    bounds: fn() -> Vec<Trait>,
    methods: fn() -> Vec<AssocFn>,
}

impl Trait {
    fn add_trait(tr: Trait) {
        let mut map = REFLECTED_TRAITS
            .get_or_init(|| RwLock::new(HashMap::new()))
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
