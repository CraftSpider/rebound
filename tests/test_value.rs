use rebound::{rebound, Type, Value};
use std::ptr::NonNull;

#[rebound]
struct TestStruct {}

#[test]
fn test_value_ty() {
    let a = 1;
    let b = TestStruct {};

    let val1 = Value::from(a);
    let val2 = Value::from(b);

    assert_eq!(val1.ty(), Type::from::<i32>());
    assert_eq!(val2.ty(), Type::from::<TestStruct>());

    assert_ne!(val1.ty(), val2.ty());
}

#[test]
fn test_value_borrow() {
    let mut val = Value::from(1);

    let borrow1 = val.borrow::<i32>();
    let borrow2 = val.borrow::<i32>();

    assert_eq!(*borrow1, 1);
    assert_eq!(*borrow1, *borrow2);

    let mut_borrow = val.borrow_mut::<i32>();

    assert_eq!(*mut_borrow, 1);
    *mut_borrow = 2;

    let norm_borrow = val.borrow::<i32>();

    assert_eq!(*norm_borrow, 2);
}

#[test]
fn test_ref_value() {
    let i = 1;
    let v = Value::from(&i);

    assert_eq!(v.ty(), Type::from::<&i32>());
    let r = v.borrow::<&i32>();
    assert_eq!(**r, 1);
}

#[test]
fn test_mut_ref_value() {
    let mut i = 1;
    let mut v = Value::from(&mut i);

    assert_eq!(v.ty(), Type::from::<&mut i32>());
    let r = v.borrow_mut::<&mut i32>();
    **r = 2;

    drop(v);

    assert_eq!(i, 2);
}

#[test]
fn test_slice_value() {
    let slice = Box::<[i32]>::from(&[1, 2, 3] as &[i32]);
    let v = unsafe { Value::from_ptr_owned(NonNull::from(Box::leak(slice))) };

    assert_eq!(v.ty(), Type::from::<[i32]>());
    let r = v.borrow::<[i32]>();
    assert_eq!(r[0], 1);
    assert_eq!(r[1], 2);
    assert_eq!(r[2], 3);
}

static mut DROP_FLAG: bool = false;

#[test]
fn test_value_drop() {
    #[rebound]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            unsafe { DROP_FLAG = true };
        }
    }

    let val = Value::from(Foo);

    assert_eq!(unsafe { DROP_FLAG }, false);

    drop(val);

    assert_eq!(unsafe { DROP_FLAG }, true);
}
