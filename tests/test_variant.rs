use rebound::{rebound, Type, Value};

#[rebound]
enum TestEnum {
    A,
    B(i32),
    C { foo: f32 },
}

#[test]
fn test_is_variant_a() {
    let v = Value::from(TestEnum::A);
    if let Type::Enum(ty) = v.ty() {
        let variant = &ty.variants()[0];
        assert!(variant
            .is_variant(&v)
            .expect("Value wasn't of the right type"))
    } else {
        panic!("TestEnum wasn't reflected correctly")
    }
}

#[test]
fn test_is_variant_b() {
    let v = Value::from(TestEnum::B(1));
    if let Type::Enum(ty) = v.ty() {
        let variant = &ty.variants()[1];
        assert!(variant
            .is_variant(&v)
            .expect("Value wasn't of the right type"))
    } else {
        panic!("TestEnum wasn't reflected correctly")
    }
}

#[test]
fn test_is_variant_c() {
    let v = Value::from(TestEnum::C { foo: 1.0 });
    if let Type::Enum(ty) = v.ty() {
        let variant = &ty.variants()[2];
        assert!(variant
            .is_variant(&v)
            .expect("Value wasn't of the right type"))
    } else {
        panic!("TestEnum wasn't reflected correctly")
    }
}
