
use rebound::{Value, TypeInfo};
use rebound_proc::rebound;
use std::marker::PhantomData;

#[rebound]
struct TestStruct {}

#[rebound]
struct TestRef<'a>(&'a i32);

#[rebound]
struct TestPhantom<'a>(PhantomData<&'a ()>);

#[test]
fn test_value_ty() {
    let a = 1;
    let b = TestStruct {};

    let val1 = Value::from(a);
    let val2 = Value::from(b);

    assert_eq!(val1.ty(), TypeInfo::from::<i32>());
    assert_eq!(val2.ty(), TypeInfo::from::<TestStruct>());

    assert_ne!(val1.ty(), val2.ty());

    // TODO: Remove this once drop is functioning
    unsafe { val1.cast::<i32>() };
    unsafe { val2.cast::<TestStruct>() };
}

#[test]
fn test_value_borrow() {
    let mut val = Value::from(1);

    let borrow1 = val.borrow::<i32>();
    let borrow2 = val.borrow::<i32>();

    assert_eq!(*borrow1, 1);
    assert_eq!(*borrow1, *borrow2);

    let mut_borrow = val.mut_borrow::<i32>();

    assert_eq!(*mut_borrow, 1);
    *mut_borrow = 2;

    let norm_borrow = val.borrow::<i32>();

    assert_eq!(*norm_borrow, 2);

    // TODO: Remove this once drop is functioning
    unsafe { val.cast::<i32>() };
}
