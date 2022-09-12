//! Some helpful utilities for working with reflected Types in various situations

use crate::Reflected;

use once_cell::sync::OnceCell;
use std::any::TypeId;
use std::collections::BTreeMap;
use std::sync::RwLock;

/// A helper for making `static` variables in a generic which are unique per type used in the
/// generic.
pub struct StaticTypeMap<T: 'static> {
    map: RwLock<BTreeMap<TypeId, &'static OnceCell<T>>>,
}

impl<T: 'static> StaticTypeMap<T> {
    /// Initialize a new `StaticTypeMap`
    pub const fn new() -> StaticTypeMap<T> {
        StaticTypeMap {
            map: RwLock::new(BTreeMap::new()),
        }
    }

    fn call_once_erased<F>(&'static self, f: F, ty_id: TypeId) -> &'static T
    where
        F: FnOnce() -> T,
    {
        let cell = {
            let reader = self.map.read().unwrap();
            reader.get(&ty_id).copied() // Copy reference
        };

        if let Some(cell) = cell {
            return cell.get_or_init(f);
        }

        let cell = {
            let mut writer = self.map.write().unwrap();
            let cell = writer.entry(ty_id).or_insert_with(|| {
                let boxed = Box::new(OnceCell::new());
                Box::leak(boxed)
            });
            *cell
        };

        cell.get_or_init(f)
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
        Self::call_once_erased(self, f, TypeId::of::<Ty::Key>())
    }
}

impl<T: 'static> Default for StaticTypeMap<T> {
    fn default() -> StaticTypeMap<T> {
        StaticTypeMap::new()
    }
}
