//! Some helpful utilities for working with reflected Types in various situations

use crate::Reflected;

use std::collections::HashMap;
use std::lazy::SyncOnceCell;
use std::sync::RwLock;

/// A helper for making `static` variables in a generic which are unique per type used in the
/// generic.
pub struct StaticTypeMap<T: 'static> {
    map: RwLock<HashMap<String, &'static SyncOnceCell<T>>>,
}

impl<T: 'static> StaticTypeMap<T> {
    /// Initialize a new `StaticTypeMap`
    pub fn new() -> StaticTypeMap<T> {
        StaticTypeMap {
            map: RwLock::new(HashMap::new()),
        }
    }

    /// Get or init the value related to a specific type in a thread-safe manner. The closure passed
    /// to this function will be run at most once per Ty over the life of the program.
    ///
    /// # Panics
    ///
    /// If we fail to acquire the backing lock for the internal map
    pub fn call_once<Ty, F>(&'static self, f: F) -> &'static T
    where
        Ty: ?Sized + Reflected,
        F: FnOnce() -> T,
    {
        let cell = {
            let reader = self.map.read().unwrap();
            reader.get(&Ty::name()).copied() // Copy reference
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
