use crate::info::*;

use hashbrown::HashMap;

static mut REFLECTED_TRAITS: Option<HashMap<String, Box<TraitInfo>>> = None;

pub type Trait = &'static TraitInfo;

#[derive(Debug)]
pub struct TraitInfo {
    name: String,
    bounds: Vec<Trait>,
    methods: fn() -> Vec<AssocFn>,
}

impl TraitInfo {
    fn ensure_statics() {
        unsafe {
            if let None = REFLECTED_TRAITS {
                REFLECTED_TRAITS = Some(HashMap::new())
            }
        }
    }

    fn add_trait(tr: TraitInfo) {
        TraitInfo::ensure_statics();

        let map;
        unsafe {
            map = REFLECTED_TRAITS
                .as_mut()
                .expect("REFLECTED_TYS not initialized correctly");
        }

        let name = tr.name().to_string();

        if map.contains_key(&name) {
            panic!("Type {} already registered", name);
        }

        map.insert(name, Box::new(tr));
    }

    pub unsafe fn new_trait(name: String, bounds: Vec<Trait>, methods: fn() -> Vec<AssocFn>) {
        let tr = TraitInfo {
            name,
            bounds,
            methods
        };

        TraitInfo::add_trait(tr);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bounds(&self) -> &Vec<Trait> {
        &self.bounds
    }
}

//#[rebound]
trait Foo {
    fn meh(&self);
}

// TODO: How are traits reflected? This is needed to support dropping Values, as well as
//       things like formatting. Can't assume things will be `dyn`able
