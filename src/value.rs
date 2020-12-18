//! Dynamically typed, lifetime safe values

use crate::{Type, Reflected, Error};

use core::ptr;
use core::marker::PhantomData;

#[derive(Debug)]
enum ValueKind {
    Owned { drop: fn(*mut()) },
    Borrowed,
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
    ptr: *mut (),
    ty: Type,
    _phantom: PhantomData<&'a ()>,
    kind: ValueKind,
}

// TODO: Count borrows safely, RefCell style?
impl<'a> Value<'a> {

    /// Create a new borrowed Value from a reference, with a lifetime no greater than that of the
    /// provided reference.
    pub fn from_ref<T: Reflected>(val: &T) -> Value {
        Value {
            ptr: val as *const T as *mut (),
            ty: Type::from::<T>(),
            _phantom: PhantomData,
            kind: ValueKind::Borrowed
        }
    }

    /// Get the raw backing pointer of this Value
    ///
    /// # Safety
    ///
    /// Similar to [`Box<T>`], the pointer returned by this function may outlive the borrowed Value,
    /// it is the user's responsibility to not use it past the lifetime of this Value.
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
                Err((self, Error::WrongType))
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
    /// ```
    /// let int = Value::from(1);
    ///
    /// // Succeeds
    /// let i = int.try_borrow::<i32>();
    /// // Fails
    /// let i = int.try_borrow::<&str>();
    /// ```
    pub fn try_borrow<T: Reflected>(&self) -> Result<&T, Error> {
        if Type::from::<T>() != self.ty() {
            Err(Error::WrongType)
        } else {
            unsafe { Ok(&*(self.ptr as *const T)) }
        }
    }

    /// Attempt to immutably borrow the T contained in this value, panicking on failure. This will
    /// panic in all the cases that [`Value::try_borrow`] would return an Err value.
    ///
    /// # Example
    ///
    /// ```
    /// let bool = Value::from(true);
    ///
    /// // Succeeds
    /// let b = bool.borrow::<bool>();
    /// // Panics
    /// let b = bool.borrow::<char>();
    /// ```
    pub fn borrow<T: Reflected>(&self) -> &T {
        self.try_borrow()
            .expect(&format!("Couldn't borrow Value as type {}", T::name()))
    }

    /// Attempt to mutably borrow the T contained by this Value in a fallible manner.
    ///
    /// This will fail if the T isn't the same as the type of this Value with [`Error::WrongType`]
    ///
    /// # Example
    ///
    /// ```
    /// let char = Value::from('a');
    ///
    /// // Succeeds
    /// let c = char.try_borrow_mut::<char>();
    /// *c = 'b';
    /// // Fails
    /// let c = char.try_borrow_mut::<i32>();
    /// *c = 2;
    ///
    /// ```
    pub fn try_borrow_mut<T: Reflected>(&mut self) -> Result<&mut T, Error> {
        if Type::from::<T>() != self.ty() {
            Err(Error::WrongType)
        } else {
            unsafe { Ok(&mut *(self.ptr as *mut T)) }
        }
    }

    /// Attempt to mutably borrow the T contained in this value, panicking on failure. This will
    /// panic in all the cases that [`Value::try_borrow_mut`] would return an Err value.
    pub fn borrow_mut<T: Reflected>(&mut self) -> &mut T {
        self.try_borrow_mut()
            .expect(&format!("Couldn't mutably borrow Value as type {}", T::name()))
    }
}

impl<'a, T: Reflected + 'a> From<T> for Value<'a> {
    fn from(val: T) -> Value<'a> {
        let ptr = Box::into_raw(Box::from(val)) as *mut ();

        Value {
            ptr,
            ty: Type::from::<T>(),
            _phantom: PhantomData,
            kind: ValueKind::Owned { drop: |ptr| {
                unsafe { Box::from_raw(ptr as *mut T) };
            }}
        }
    }
}

impl<'a> Drop for Value<'a> {
    fn drop(&mut self) {
        if let ValueKind::Owned { drop } = &self.kind {
            if !self.ptr.is_null() {
                drop(self.ptr);
            }
        }
    }
}
