use crate::reflect::*;
use crate::{Field, Type};

use core::slice::*;

impl<T: Reflected> Reflected for Chunks<'_, T> {
    fn name() -> String {
        format!("core::slice::Chunks<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<Chunks<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for Chunks<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl<T: Reflected> Reflected for ChunksExact<'_, T> {
    fn name() -> String {
        format!("core::slice::ChunksExact<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<ChunksExact<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for ChunksExact<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl<T: Reflected> Reflected for ChunksExactMut<'_, T> {
    fn name() -> String {
        format!("core::slice::ChunksExactMut<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<ChunksExactMut<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for ChunksExactMut<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl<T: Reflected> Reflected for ChunksMut<'_, T> {
    fn name() -> String {
        format!("core::slice::ChunksMut<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<ChunksMut<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for ChunksMut<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl<T: Reflected> Reflected for Iter<'_, T> {
    fn name() -> String {
        format!("core::slice::Iter<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<Iter<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for Iter<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl<T: Reflected> Reflected for IterMut<'_, T> {
    fn name() -> String {
        format!("core::slice::IterMut<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<IterMut<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for IterMut<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl<T: Reflected> Reflected for RChunks<'_, T> {
    fn name() -> String {
        format!("core::slice::RChunks<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<RChunks<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for RChunks<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl<T: Reflected> Reflected for RChunksExact<'_, T> {
    fn name() -> String {
        format!("core::slice::RChunksExact<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<RChunksExact<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for RChunksExact<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl<T: Reflected> Reflected for RChunksExactMut<'_, T> {
    fn name() -> String {
        format!("core::slice::RChunksExactMut<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<RChunksExactMut<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for RChunksExactMut<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl<T: Reflected> Reflected for RChunksMut<'_, T> {
    fn name() -> String {
        format!("core::slice::RChunksMut<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<RChunksMut<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for RChunksMut<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}

impl<T: Reflected> Reflected for Windows<'_, T> {
    fn name() -> String {
        format!("core::slice::Windows<{}>", T::name())
    }

    unsafe fn init() {
        Type::new_struct::<Windows<T>>()
    }
}

impl<T: Reflected> ReflectedStruct for Windows<'_, T> {
    fn fields() -> Vec<Field> {
        vec![] // TODO
    }
}
