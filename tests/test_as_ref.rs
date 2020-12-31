use rebound::{Type, Value};

#[test]
fn val_as_ref() {
    let v = Value::from(1);
    let v2 = v.as_ref().unwrap();
    assert_eq!(v2.ty(), Type::from::<&i32>());
    let v3 = v2.as_ref().unwrap();
    assert_eq!(v3.ty(), Type::from::<&i32>());
}

#[test]
fn val_as_mut() {
    let mut v = Value::from(1);
    let v2 = v.as_mut().unwrap();
    assert_eq!(v2.ty(), Type::from::<&mut i32>());
}
