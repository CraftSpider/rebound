use rebound::{rebound, Error, Type, Value};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};

#[rebound]
struct TestStruct {}

#[test]
fn test_value_ty() {
    let a = 1;
    let b = TestStruct {};

    let val1 = Value::from(a);
    let val2 = Value::from(b);

    assert_eq!(val1.ty(), Type::of::<i32>());
    assert_eq!(val2.ty(), Type::of::<TestStruct>());

    assert_ne!(val1.ty(), val2.ty());
}

#[test]
fn test_value_cast() {
    let val = Value::from(-1i32);
    assert_eq!(val.cast::<i32>(), -1);
}

#[test]
fn test_value_cast_err() {
    let val = Value::from(-1i32);
    let (_, err) = val.try_cast::<f32>().unwrap_err();
    assert_eq!(
        err,
        Error::WrongType {
            wrong_ty: Type::of::<f32>(),
            right_ty: Type::of::<i32>()
        }
    );

    let val = Value::from_ref(&1i32);
    let (_, err) = val.try_cast::<i32>().unwrap_err();
    assert_eq!(err, Error::BorrowedValue);

    let mut temp = 1i32;
    let val = Value::from_mut(&mut temp);
    let (_, err) = val.try_cast::<i32>().unwrap_err();
    assert_eq!(err, Error::BorrowedValue);
}

#[test]
fn test_value_borrow() {
    let val = Value::from(1i32);

    let borrow1 = val.borrow::<i32>();
    let borrow2 = val.borrow::<i32>();

    assert_eq!(*borrow1, 1);
    assert_eq!(*borrow1, *borrow2);

    let val = Value::from_ref(&1i32);

    let borrow1 = val.borrow::<i32>();
    let borrow2 = val.borrow::<i32>();

    assert_eq!(*borrow1, 1);
    assert_eq!(*borrow1, *borrow2);

    let mut temp = 1i32;
    let val = Value::from_mut(&mut temp);

    let borrow1 = val.borrow::<i32>();
    let borrow2 = val.borrow::<i32>();

    assert_eq!(*borrow1, 1);
    assert_eq!(*borrow1, *borrow2);
}

#[test]
fn test_value_borrow_err() {
    let val = Value::from(1i32);

    let err = val.try_borrow::<f32>().unwrap_err();
    assert_eq!(
        err,
        Error::WrongType {
            wrong_ty: Type::of::<f32>(),
            right_ty: Type::of::<i32>()
        }
    );

    let val = Value::from_ref(&1i32);

    let err = val.try_borrow::<f32>().unwrap_err();
    assert_eq!(
        err,
        Error::WrongType {
            wrong_ty: Type::of::<f32>(),
            right_ty: Type::of::<i32>()
        }
    );

    let mut temp = 1i32;
    let val = Value::from_mut(&mut temp);

    let err = val.try_borrow::<f32>().unwrap_err();
    assert_eq!(
        err,
        Error::WrongType {
            wrong_ty: Type::of::<f32>(),
            right_ty: Type::of::<i32>()
        }
    );
}

#[test]
fn test_value_borrow_mut() {
    let mut val = Value::from(1i32);

    let mut_borrow = val.borrow_mut::<i32>();
    assert_eq!(*mut_borrow, 1);
    *mut_borrow = 2;

    let norm_borrow = val.borrow::<i32>();
    assert_eq!(*norm_borrow, 2);

    let mut temp = 1i32;
    let mut val = Value::from_mut(&mut temp);

    let mut_borrow = val.borrow_mut::<i32>();
    assert_eq!(*mut_borrow, 1);
    *mut_borrow = 2;

    let norm_borrow = val.borrow::<i32>();
    assert_eq!(*norm_borrow, 2);
}

#[test]
fn test_value_borrow_mut_err() {
    let mut val = Value::from(1i32);

    let err = val.try_borrow_mut::<f32>().unwrap_err();
    assert_eq!(
        err,
        Error::WrongType {
            wrong_ty: Type::of::<f32>(),
            right_ty: Type::of::<i32>()
        }
    );

    let mut val = Value::from_ref(&1i32);

    let err = val.try_borrow_mut::<i32>().unwrap_err();
    assert_eq!(err, Error::ImmutableValue);

    let mut temp = 1i32;
    let mut val = Value::from_mut(&mut temp);

    let err = val.try_borrow_mut::<f32>().unwrap_err();
    assert_eq!(
        err,
        Error::WrongType {
            wrong_ty: Type::of::<f32>(),
            right_ty: Type::of::<i32>()
        }
    );
}

#[test]
fn test_value_as_ref() {
    let v = Value::from(1i32);
    let mut v2 = v.as_ref().unwrap();

    assert_eq!(**v2.borrow::<&i32>(), 1);
    assert_eq!(v2.ty(), Type::of::<&i32>());
    let v3 = v2.as_ref().unwrap();

    assert_eq!(**v3.borrow::<&i32>(), 1);
    assert_eq!(v3.ty(), Type::of::<&i32>());
    assert_eq!(v2.as_mut().unwrap_err(), Error::CantReborrow);
}

#[test]
fn test_value_as_mut() {
    let mut v = Value::from(1i32);
    let mut v2 = v.as_mut().unwrap();
    assert_eq!(**v2.borrow::<&mut i32>(), 1);
    assert_eq!(v2.ty(), Type::of::<&mut i32>());
    assert_eq!(v2.as_ref().unwrap_err(), Error::CantReborrow);
    assert_eq!(v2.as_mut().unwrap_err(), Error::CantReborrow);
}

#[test]
fn test_ref_value() {
    let i = 1;
    let v = Value::from(&i);

    assert_eq!(v.ty(), Type::of::<&i32>());
    let r = v.borrow::<&i32>();
    assert_eq!(**r, 1);
}

#[test]
fn test_mut_ref_value() {
    let mut i = 1;
    let mut v = Value::from(&mut i);

    assert_eq!(v.ty(), Type::of::<&mut i32>());
    let r = v.borrow_mut::<&mut i32>();
    **r = 2;

    drop(v);

    assert_eq!(i, 2);
}

#[test]
fn test_slice_value() {
    let slice = Box::<[i32]>::from(&[1, 2, 3] as &[i32]);
    let v = unsafe { Value::from_ptr_owned(NonNull::from(Box::leak(slice))) };

    assert_eq!(v.ty(), Type::of::<[i32]>());
    let r = v.borrow::<[i32]>();
    assert_eq!(r[0], 1);
    assert_eq!(r[1], 2);
    assert_eq!(r[2], 3);
}

#[test]
fn test_value_drop() {
    static DROP_FLAG: AtomicBool = AtomicBool::new(false);

    #[rebound]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            DROP_FLAG.store(true, Ordering::SeqCst);
        }
    }

    let val = Value::from(Foo);

    assert_eq!(DROP_FLAG.load(Ordering::SeqCst), false);

    drop(val);

    assert_eq!(DROP_FLAG.load(Ordering::SeqCst), true);
}
