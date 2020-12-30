use crate::Reflected;

use std::collections::HashMap;
use std::lazy::SyncOnceCell;
use std::sync::RwLock;

pub struct StaticTypeMap<T: 'static> {
    map: RwLock<HashMap<String, &'static SyncOnceCell<T>>>,
}

impl<T: 'static> StaticTypeMap<T> {
    pub fn new() -> StaticTypeMap<T> {
        StaticTypeMap {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn call_once<Ty, F>(&'static self, f: F) -> &'static T
    where
        Ty: ?Sized + Reflected,
        F: FnOnce() -> T,
    {
        let cell = {
            let reader = self.map.read().unwrap();
            reader.get(&Ty::name()).cloned() // Clone reference
        };

        if let Some(cell) = cell {
            return cell.get_or_init(f);
        }

        let cell = {
            let mut writer = self.map.write().unwrap();
            let cell = writer.entry(Ty::name()).or_insert_with(|| {
                let boxed = Box::new(SyncOnceCell::new());
                Box::leak(boxed)
            });
            *cell
        };

        cell.get_or_init(f)
    }
}

impl<T: 'static> Default for StaticTypeMap<T> {
    fn default() -> StaticTypeMap<T> {
        StaticTypeMap::new()
    }
}
