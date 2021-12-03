//! Dynamically typed, lifetime safe values

use crate::{Error, Reflected, Type};

use crate::ty::CommonTypeInfo;
use core::fmt;
use core::marker::PhantomData;
use core::ptr;

#[derive(Debug)]
enum ValueKind {
    Owned { drop: fn(*mut (), *mut ()) },
    Borrowed,
    Moved,
}

fn drop_impl<T: ?Sized + Reflected>(meta: *mut (), ptr: *mut ()) {
    unsafe {
        let meta = *Box::from_raw(meta.cast::<<T as ptr::Pointee>::Metadata>());
        Box::from_raw(ptr::from_raw_parts_mut::<T>(ptr, meta));
    }
}

/// Trait to represent a bound that a type may not outlives some given lifetime. Used to allow
/// sound borrowing of non-static Values.
///
/// If this was allowed, one could create a value containing a reference with a lifetime of `'a`,
/// then when they call `borrow`, the compiler would allow the creation of an output reference with
/// a lifetime of `'static`, which would be immediately unsound.
///
/// # Safety
///
/// Any implementation must ensure that all generics (lifetimes or types) in the type being
/// implemented on are correctly bounded to not outlive `'no`
pub unsafe trait NotOutlives<'no> {}

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
    kind: ValueKind,
    _phantom: PhantomData<&'a ()>,
}

#[allow(clippy::should_implement_trait)]
impl<'a> Value<'a> {
    /// Create a new owned Value from a pointer, with a lifetime no greater than that of the
    /// provided type.
    ///
    /// # Safety
    ///
    /// Similar to [`Box::from_raw`], this function may result in a double-free if called more than
    /// once with the same pointer.
    pub unsafe fn from_ptr_owned<T: ?Sized + Reflected + 'a>(val: *mut T) -> Value<'a> {
        let (ptr, meta) = val.to_raw_parts();
        let meta = Box::into_raw(Box::new(meta)).cast();

        Value {
            meta,
            ptr,
            ty: Type::from::<T>(),
            kind: ValueKind::Owned {
                drop: drop_impl::<T>,
            },
            _phantom: PhantomData,
        }
    }

    /// Create a new borrowed Value from a reference, with a lifetime no greater than that of the
    /// provided reference.
    pub fn from_ref<T: ?Sized + Reflected>(val: &T) -> Value<'_> {
        let (ptr, meta) = (val as *const T as *mut T).to_raw_parts();
        let meta = Box::into_raw(Box::new(meta)).cast();

        Value {
            meta,
            ptr,
            ty: Type::from::<T>(),
            kind: ValueKind::Borrowed,
            _phantom: PhantomData,
        }
    }

    /// Get a pointer to the raw backing metadata of this Value
    ///
    /// # Safety
    ///
    /// Similar to [`pointer::as_ref`], the pointer returned by this function may outlive the
    /// borrowed Value, it is the user's responsibility to not use it past the lifetime of this
    /// Value.
    pub unsafe fn raw_meta(&self) -> *mut () {
        self.meta
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

    /// Attempt to move the contained T out of this Value in a fallible manner. This method is
    /// unsafe due to possible lifetime unsoundness, use [`Value::try_cast`] for a lifetime-safe
    /// variant.
    ///
    /// # Errors
    ///
    /// - This will fail if the Value is Borrowed with an [`Error::BorrowedValue`]
    /// - This will fail if the T isn't the same as the type of this value with [`Error::WrongType`]
    ///
    /// # Safety
    ///
    /// If this Value contains data which may not live forever, this function does not ensure that
    /// the provided T does *not* outlive `'a`. As such, it is the user's responsibility to not move
    /// data out of this value in a way which gives it a lifetime longer than its original.
    pub unsafe fn try_cast_unsafe<T: Reflected>(mut self) -> Result<T, (Self, Error)> {
        if let ValueKind::Owned { .. } = &self.kind {
            if Type::from::<T>() == self.ty {
                self.kind = ValueKind::Moved;
                // Want to drop meta here as well, else we'll leak it
                Box::from_raw(self.meta);
                Ok(*Box::from_raw(self.ptr.cast::<T>()))
            } else {
                let ty = self.ty;
                Err((self, Error::wrong_type(Type::from::<T>(), ty)))
            }
        } else {
            Err((self, Error::BorrowedValue))
        }
    }

    /// Attempt to move the contained T out of this Value, panicking on failure. This will panic
    /// in all the cases that [`Value::try_case_unsafe`] would return an Err value.
    ///
    /// # Panics
    ///
    /// If this value contains some type other than `T`, or if this is a borrowed value and as
    /// such cannot be moved out of
    ///
    /// # Safety
    ///
    /// See [`Value::try_cast`]
    pub unsafe fn cast_unsafe<T: Reflected>(self) -> T {
        self.try_cast_unsafe()
            .unwrap_or_else(|_| panic!("Couldn't cast Value into type {}", T::name()))
    }

    /// Attempt to move the contained T out of this Value in a fallible manner.
    ///
    /// # Errors
    ///
    /// - This will fail if the Value is Borrowed with an [`Error::BorrowedValue`]
    /// - This will fail if the T isn't the same as the type of this value with [`Error::WrongType`]
    pub fn try_cast<T: Reflected + NotOutlives<'a>>(self) -> Result<T, (Self, Error)> {
        unsafe { self.try_cast_unsafe() }
    }

    /// Attempt to move the contained T out of this Value, panicking on failure. This will panic
    /// in all the cases that [`Value::try_cast`] would return an Err value.
    ///
    /// # Panics
    ///
    /// If this value contains some type other than `T`, or if this is a borrowed value and as
    /// such cannot be moved out of
    ///
    pub fn cast<T: Reflected + NotOutlives<'a>>(self) -> T {
        self.try_cast()
            .unwrap_or_else(|_| panic!("Couldn't cast Value into type {}", T::name()))
    }

    // Should the returned references live for 'a? No. Why?
    // Assume an owned `Value<'static>` with ty of i32. A borrow out may live forever, however,
    // the Value will destroy the contained data when it goes out of scope. This means a user
    // could safely borrow a value, and then have it become invalid.

    /// Attempt to immutable borrow the T contained in this Value in a fallible manner.
    ///
    /// This will fail if the T isn't the same as the type of this value with [`Error::WrongType`]
    ///
    /// # Safety
    ///
    /// See [`Value::try_cast`]
    pub unsafe fn try_borrow_unsafe<T: ?Sized + Reflected>(&self) -> Result<&T, Error> {
        if Type::from::<T>() == self.ty() {
            let ptr = ptr::from_raw_parts_mut(self.ptr, *self.meta.cast());
            Ok(&*ptr)
        } else {
            Err(Error::wrong_type(Type::from::<T>(), self.ty()))
        }
    }

    /// Attempt to immutably borrow the T contained in this value, panicking on failure.
    ///
    /// # Panics
    ///
    /// In all cases that [`Value::try_borrow_unsafe`] would return an Err, if this value contains
    /// some type other than `T`
    ///
    /// # Safety
    ///
    /// See [`Value::try_cast`]
    pub unsafe fn borrow_unsafe<T: ?Sized + Reflected>(&self) -> &T {
        self.try_borrow_unsafe()
            .unwrap_or_else(|_| panic!("Couldn't borrow Value as type {}", T::name()))
    }

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
    pub fn try_borrow<'b, T: ?Sized + Reflected + NotOutlives<'a>>(
        &'b self,
    ) -> Result<&'b T, Error> {
        unsafe { self.try_borrow_unsafe() }
    }

    /// Attempt to immutably borrow the T contained in this value, panicking on failure. This will
    /// panic in all the cases that [`Value::try_borrow`] would return an Err value.
    ///
    /// # Panics
    ///
    /// In all cases that [`Value::try_borrow`] would return an Err, if this value contains some
    /// type other than `T`
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
    pub fn borrow<'b, T: ?Sized + Reflected + NotOutlives<'a>>(&'b self) -> &'b T {
        self.try_borrow()
            .unwrap_or_else(|_| panic!("Couldn't borrow Value as type {}", T::name()))
    }

    /// Attempt to mutably borrow the T contained by this Value in a fallible manner.
    ///
    /// This will fail if the T isn't the same as the type of this Value with [`Error::WrongType`]
    ///
    /// # Safety
    ///
    /// See [`Value::try_cast`]
    pub unsafe fn try_borrow_unsafe_mut<T: ?Sized + Reflected>(&mut self) -> Result<&mut T, Error> {
        if Type::from::<T>() == self.ty() {
            let ptr = ptr::from_raw_parts_mut(self.ptr, *self.meta.cast());
            Ok(&mut *ptr)
        } else {
            Err(Error::wrong_type(Type::from::<T>(), self.ty()))
        }
    }

    /// Attempt to mutably borrow the T contained in this value, panicking on failure. This will
    /// panic in all the cases that [`Value::try_borrow_mut`] would return an Err value.
    ///
    /// # Panics
    ///
    /// In all cases that [`Value::try_borrow_unsafe_mut`] would return an Err, if this value
    /// contains some type other than `T`
    ///
    /// # Safety
    ///
    /// See [`Value::try_cast`]
    pub unsafe fn borrow_unsafe_mut<T: ?Sized + Reflected>(&mut self) -> &mut T {
        self.try_borrow_unsafe_mut()
            .unwrap_or_else(|_| panic!("Couldn't mutably borrow Value as type {}", T::name()))
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
    pub fn try_borrow_mut<'b, T: ?Sized + Reflected + NotOutlives<'a>>(
        &'b mut self,
    ) -> Result<&'b mut T, Error> {
        unsafe { self.try_borrow_unsafe_mut() }
    }

    /// Attempt to mutably borrow the T contained in this value, panicking on failure. This will
    /// panic in all the cases that [`Value::try_borrow_mut`] would return an Err value.
    ///
    /// # Panics
    ///
    /// In all cases that [`Value::try_borrow_mut`] would return an Err, if this value
    /// contains some type other than `T`
    ///
    /// # Example
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
    pub fn borrow_mut<'b, T: ?Sized + Reflected + NotOutlives<'a>>(&'b mut self) -> &'b mut T {
        self.try_borrow_mut()
            .unwrap_or_else(|_| panic!("Couldn't mutably borrow Value as type {}", T::name()))
    }

    /// If this Value is not a reference Type, get a Value containing an immutable reference to
    /// the data contained within this Value. A convenience function for operations that expect a
    /// reference to data you own.
    pub fn as_ref(&self) -> Result<Value<'_>, Error> {
        self.ty.as_ref(self)
    }

    /// If this Value is not a reference Type, get a Value containing a mutable reference to the
    /// data contained within this Value. A convenience function for operations that expect a
    /// mutable reference to data you own.
    pub fn as_mut(&mut self) -> Result<Value<'_>, Error> {
        let ty = self.ty;
        ty.as_mut(self)
    }
}

impl<'a> fmt::Pointer for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

impl<'a, T: Reflected + 'a> From<T> for Value<'a> {
    default fn from(val: T) -> Value<'a> {
        let (ptr, meta) = Box::into_raw(Box::new(val)).to_raw_parts();
        let meta = Box::into_raw(Box::<<T as ptr::Pointee>::Metadata>::new(meta)).cast();

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
            drop(self.meta, self.ptr);
        }
    }
}
