use crate::{Type, Reflected, Error};

use core::ptr;
use core::marker::PhantomData;

// Similar to Any, but more powerful
#[derive(Debug)]
pub enum Value<'a> {
    Owned {
        ptr: *mut (),
        ty: Type,
        drop_impl: fn(*mut ()),
        _phantom: PhantomData<&'a ()>
    },
    Borrowed {
        ptr: *mut (),
        ty: Type,
        _phantom: PhantomData<&'a ()>
    },
}

// TODO: Count borrows safely, RefCell style?
impl<'a> Value<'a> {
    // TODO: This probably needs a 'static bound to be WF. Same expressiveness problem as
    //       ReflectedTuple
    pub fn from<'b, T: Reflected + 'b>(val: T) -> Value<'b> {
        let ptr = Box::into_raw(Box::from(val)) as *mut ();

        Value::Owned {
            ptr,
            ty: Type::from::<T>(),
            drop_impl: |ptr| {
                unsafe { Box::from_raw(ptr as *mut T) };
            },
            _phantom: PhantomData,
        }
    }

    pub fn from_ref<T: Reflected>(val: &T) -> Value {
        Value::Borrowed {
            ptr: val as *const T as *mut (),
            ty: Type::from::<T>(),
            _phantom: PhantomData,
        }
    }

    fn ptr(&self) -> *mut () {
        match self {
            Value::Owned { ptr, .. }
            | Value::Borrowed { ptr, .. } => *ptr
        }
    }

    pub fn ty(&self) -> Type {
        match self {
            Value::Owned { ty, ..}
            | Value::Borrowed { ty, .. } => *ty
        }
    }

    // This must be 'static and Owned because otherwise we could just magic up a `&'static T` from a
    // `&'a T`, even if 'a does not live for `'static`.
    // FIXME: Still unsound, we can transmute a Foo<'a> to a Foo<'static> through this.
    //        If only I could constrain the output to the struct `'a`
    pub unsafe fn try_cast<T: Reflected>(mut self) -> Result<T, (Self, Error)> {
        if let Value::Owned { ptr, ty, .. } = &mut self {
            if Type::from::<T>() != *ty {
                Err((self, Error::WrongType))
            } else {
                let old_ptr = *ptr;
                *ptr = ptr::null_mut();
                Ok(*Box::from_raw(old_ptr as *mut T))
            }
        } else {
            Err((self, Error::InvalidValue))
        }
    }

    pub unsafe fn cast<T: Reflected + 'static>(self) -> T {
        self.try_cast()
            .expect(&format!("Couldn't cast Value into type {}", T::name()))
    }

    pub fn try_borrow<T: Reflected>(&self) -> Result<&'a T, Error> {
        if Type::from::<T>() != self.ty() {
            Err(Error::WrongType)
        } else {
            unsafe { Ok(&*(self.ptr() as *const T)) }
        }
    }

    pub fn borrow<T: Reflected>(&self) -> &'a T {
        self.try_borrow()
            .expect(&format!("Couldn't borrow Value as type {}", T::name()))
    }

    pub fn try_mut_borrow<T: Reflected>(&mut self) -> Result<&'a mut T, Error> {
        if Type::from::<T>() != self.ty() {
            Err(Error::WrongType)
        } else {
            unsafe { Ok(&mut *(self.ptr() as *mut T)) }
        }
    }

    pub fn mut_borrow<T: Reflected>(&mut self) -> &'a mut T {
        self.try_mut_borrow()
            .expect(&format!("Couldn't mutably borrow Value as type {}", T::name()))
    }
}

impl<'a> Drop for Value<'a> {
    fn drop(&mut self) {
        if let Value::Owned { ptr, drop_impl, .. } = self {
            if !ptr.is_null() {
                drop_impl(*ptr);
            }
        }
    }
}
