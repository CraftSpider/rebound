use rebound::{Type, Value};

#[test]
fn val_as_ref() {
    let v = Value::from(1i32);
    let mut v2 = v.as_ref().unwrap();

    assert_eq!(**v2.borrow::<&i32>(), 1);
    assert_eq!(v2.ty(), Type::from::<&i32>());
    let v3 = v2.as_ref().unwrap();

    assert_eq!(**v3.borrow::<&i32>(), 1);
    assert_eq!(v3.ty(), Type::from::<&i32>());
    v2.as_mut().unwrap_err();
}

#[test]
fn val_as_mut() {
    let mut v = Value::from(1i32);
    let mut v2 = v.as_mut().unwrap();
    assert_eq!(**v2.borrow::<&mut i32>(), 1);
    assert_eq!(v2.ty(), Type::from::<&mut i32>());
    v2.as_ref().unwrap_err();
    v2.as_mut().unwrap_err();
}
