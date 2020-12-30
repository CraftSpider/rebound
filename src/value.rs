//! Dynamically typed, lifetime safe values

use crate::{Error, Reflected, Type};

use core::marker::PhantomData;
use core::ptr;
use core::fmt;

#[derive(Debug)]
enum ValueKind {
    Owned { drop: fn(*mut(), *mut ()) },
    Borrowed,
}

fn drop_impl<T: ?Sized + Reflected>(meta: *mut(), ptr: *mut()) {
    unsafe {
        let meta = Box::from_raw(meta.cast::<T::Meta>());
        Box::from_raw(T::assemble(*meta, ptr));
    }
}

/// A Value represents a value with an erased type. It may be owned or borrowed.
/// The Value will have at most the lifetime of the value it was created from.
///
/// An owned Value will safely drop the contained object when it dies, while a borrowed Value
/// will not.
///
/// A Value may be borrowed out as a concrete type, though an attempt to do so
/// as a type not the same as the input type will result in a failure at runtime.
#[derive(Debug)]
pub struct Value<'a> {
    meta: *mut (),
    ptr: *mut (),
    ty: Type,
    _phantom: PhantomData<&'a ()>,
    kind: ValueKind,
}

impl<'a> Value<'a> {
    /// Create a new owned Value from a pointer, with a lifetime no greater than that of the
    /// provided type.
    ///
    /// # Safety
    ///
    /// Similar to [`Box::from_raw`], this function may result in a double-free if called more than
    /// once with the same pointer.
    pub unsafe fn from_ptr_owned<T: ?Sized + Reflected + 'a>(val: *mut T) -> Value<'a> {
        let meta = Box::into_raw(Box::new(val.disassemble().0)).cast();
        let ptr = val.cast();

        Value {
            meta,
            ptr,
            ty: Type::from::<T>(),
            _phantom: PhantomData,
            kind: ValueKind::Owned {
                drop: drop_impl::<T>,
            },
        }
    }

    /// Create a new borrowed Value from a reference, with a lifetime no greater than that of the
    /// provided reference.
    pub fn from_ref<T: ?Sized + Reflected>(val: &T) -> Value {
        let (_, ptr) = T::disassemble(val);

        Value {
            meta: ptr::null_mut(),
            ptr,
            ty: Type::from::<T>(),
            _phantom: PhantomData,
            kind: ValueKind::Borrowed,
        }
    }

    /// Get the raw backing pointer of this Value
    ///
    /// # Safety
    ///
    /// Similar to [`pointer::as_ref`], the pointer returned by this function may outlive the
    /// borrowed Value, it is the user's responsibility to not use it past the lifetime of this
    /// Value.
    pub unsafe fn raw_ptr(&self) -> *mut () {
        self.ptr
    }

    /// Get the [`Type`] of this Value
    pub fn ty(&self) -> Type {
        self.ty
    }

    // FIXME: Still unsound, we can transmute a Foo<'a> to a Foo<'static> through this.
    //        If only I could constrain the output to the struct `'a`
    /// Attempt to move the contained T out of this Value in a fallible manner.
    ///
    /// - This will fail if the Value is Borrowed with an [`Error::InvalidValue`]
    /// - This will fail if the T isn't the same as the type of this value with [`Error::WrongType`]
    ///
    /// # Safety
    ///
    /// If this Value contains data which may not live forever, there is no way to ensure that the
    /// provided T does not outlive `'a`. As such, it is the user's responsibility to not move data
    /// out of this value in a way which gives it a lifetime longer than its original.
    pub unsafe fn try_cast<T: Reflected>(mut self) -> Result<T, (Self, Error)> {
        if let ValueKind::Owned { .. } = &self.kind {
            if Type::from::<T>() != self.ty {
                let ty = self.ty;
                Err((self, Error::wrong_type(Type::from::<T>(), ty)))
            } else {
                let old_ptr = self.ptr;
                self.ptr = ptr::null_mut();
                Ok(*Box::from_raw(old_ptr as *mut T))
            }
        } else {
            Err((self, Error::InvalidValue))
        }
    }

    /// Attempt to move the contained T out of this Value, panicking on failure. This will panic
    /// in all the cases that [`Value::try_cast`] would return an Err value.
    ///
    /// # Safety
    ///
    /// See [`Value::try_cast`]
    pub unsafe fn cast<T: Reflected>(self) -> T {
        self.try_cast()
            .expect(&format!("Couldn't cast Value into type {}", T::name()))
    }

    // Should the returned references live for 'a? No. Why?
    // Assume an owned `Value<'static>` with ty of i32. A borrow out may live forever, however,
    // the Value will destroy the contained data when it goes out of scope. This means a user
    // could safely borrow a value, and then have it become invalid.

    /// Attempt to immutably borrow the T contained by this Value in a fallible manner.
    ///
    /// This will fail if the T isn't the same as the type of this value with [`Error::WrongType`]
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rebound::Value;
    /// let int = Value::from(1);
    ///
    /// // Succeeds
    /// let i = int.try_borrow::<i32>();
    /// // Fails
    /// let i = int.try_borrow::<&str>();
    /// ```
    pub fn try_borrow<T: ?Sized + Reflected>(&self) -> Result<&T, Error> {
        if Type::from::<T>() != self.ty() {
            Err(Error::wrong_type(Type::from::<T>(), self.ty()))
        } else {
            let ptr = T::assemble(unsafe { *Box::from_raw(self.meta.cast::<T::Meta>()) }, self.ptr);
            unsafe { Ok(&*ptr) }
        }
    }

    /// Attempt to immutably borrow the T contained in this value, panicking on failure. This will
    /// panic in all the cases that [`Value::try_borrow`] would return an Err value.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rebound::Value;
    /// let bool = Value::from(true);
    ///
    /// // Succeeds
    /// let b = bool.borrow::<bool>();
    /// // Panics
    /// let b = bool.borrow::<char>();
    /// ```
    pub fn borrow<T: ?Sized + Reflected>(&self) -> &T {
        self.try_borrow()
            .expect(&format!("Couldn't borrow Value as type {}", T::name()))
    }

    /// Attempt to mutably borrow the T contained by this Value in a fallible manner.
    ///
    /// This will fail if the T isn't the same as the type of this Value with [`Error::WrongType`]
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rebound::Value;
    /// let mut char = Value::from('a');
    ///
    /// // Succeeds
    /// let c = char.try_borrow_mut::<char>()
    ///     .unwrap();
    /// *c = 'b';
    /// // Fails
    /// let c = char.try_borrow_mut::<i32>()
    ///     .unwrap();
    /// *c = 2;
    ///
    /// ```
    pub fn try_borrow_mut<T: ?Sized + Reflected>(&mut self) -> Result<&mut T, Error> {
        if Type::from::<T>() != self.ty() {
            Err(Error::wrong_type(Type::from::<T>(), self.ty()))
        } else {
            let ptr = T::assemble(unsafe { *Box::from_raw(self.meta.cast::<T::Meta>()) }, self.ptr);
            unsafe { Ok(&mut *ptr) }
        }
    }

    /// Attempt to mutably borrow the T contained in this value, panicking on failure. This will
    /// panic in all the cases that [`Value::try_borrow_mut`] would return an Err value.
    ///
    /// ```no_run
    /// # use rebound::Value;
    /// let mut str = Value::from("a string");
    ///
    /// // Succeeds
    /// let s = str.borrow_mut::<&str>();
    /// // Fails
    /// let s = str.borrow_mut::<&i32>();
    /// ```
    pub fn borrow_mut<T: ?Sized + Reflected>(&mut self) -> &mut T {
        self.try_borrow_mut().expect(&format!(
            "Couldn't mutably borrow Value as type {}",
            T::name()
        ))
    }
}

impl<'a> fmt::Pointer for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

impl<'a, T: Reflected + 'a> From<T> for Value<'a> {
    fn from(val: T) -> Value<'a> {
        let meta = Box::into_raw(Box::new(val.disassemble().0)).cast();
        let ptr = Box::into_raw(Box::new(val)).cast();

        Value {
            meta,
            ptr,
            ty: Type::from::<T>(),
            _phantom: PhantomData,
            kind: ValueKind::Owned {
                drop: drop_impl::<T>,
            },
        }
    }
}

impl<'a> Drop for Value<'a> {
    fn drop(&mut self) {
        if let ValueKind::Owned { drop } = &self.kind {
            if !self.ptr.is_null() {
                drop(self.meta, self.ptr);
            }
        }
    }
}
