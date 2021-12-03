use rebound::{rebound, Type, Value, Variant};

#[rebound]
struct Struct {
    field_a: f64,
}

#[rebound]
enum Enum {
    A,
    B(f64),
    C { is_thing: bool },
}

#[test]
fn test_access_struct() {
    let val = Value::from(Struct { field_a: -1.0 });
    if let Type::Struct(ty) = val.ty() {
        let field = &ty.fields()[0];
        let a = field
            .get_ref(&val)
            .expect("Couldn't get value from Struct::field_a");

        assert_eq!(a.ty(), field.ty());
        assert_eq!(*a.borrow::<f64>(), -1.0);
    } else {
        panic!("Struct wasn't reflected correctly")
    }
}

#[test]
fn test_set_struct() {
    let mut val = Value::from(Struct { field_a: -1.0 });
    if let Type::Struct(ty) = val.ty() {
        let field = &ty.fields()[0];
        field
            .set(&mut val, Value::from(10.5))
            .expect("Couldn't set value of Struct::field_a");

        assert_eq!(val.borrow::<Struct>().field_a, 10.5);
    } else {
        panic!("Struct wasn't reflected correctly")
    }
}

#[test]
fn test_access_enum() {
    if let Type::Enum(ty) = Type::from::<Enum>() {
        let val = Value::from(Enum::B(-10.0));
        if let Variant::Tuple(info) = &ty.variants()[1] {
            let field = &info.fields()[0];
            let f = field
                .get_ref(&val)
                .expect("Couldn't get value from Enum::B(f64) field");

            assert_eq!(f.ty(), field.ty());
            assert_eq!(*f.borrow::<f64>(), -10.0);
        } else {
            panic!("Enum wasn't reflected properly")
        }

        let val = Value::from(Enum::C { is_thing: false });
        if let Variant::Struct(info) = &ty.variants()[2] {
            let field = &info.fields()[0];
            let f = field
                .get_ref(&val)
                .expect("Couldn't get value from Enum::C { is_thing } field");

            assert_eq!(f.ty(), field.ty());
            assert_eq!(*f.borrow::<bool>(), false);
        } else {
            panic!("Enum wasn't reflected properly")
        }
    } else {
        panic!("Enum wasn't reflected properly")
    }
}

#[test]
fn test_set_enum() {
    if let Type::Enum(ty) = Type::from::<Enum>() {
        let mut val = Value::from(Enum::B(-10.0));
        if let Variant::Tuple(info) = &ty.variants()[1] {
            let field = &info.fields()[0];
            field
                .set(&mut val, Value::from(123.45))
                .expect("Couldn't set value of Enum::B(f64) field");

            assert!(matches!(*val.borrow::<Enum>(), Enum::B(f) if f == 123.45));
        } else {
            panic!("Enum wasn't reflected properly")
        }

        let mut val = Value::from(Enum::C { is_thing: false });
        if let Variant::Struct(info) = &ty.variants()[2] {
            let field = &info.fields()[0];
            field
                .set(&mut val, Value::from(true))
                .expect("Couldn't set value of Enum::C { is_thing } field");

            assert!(matches!(*val.borrow::<Enum>(), Enum::C { is_thing } if is_thing == true));
        } else {
            panic!("Enum wasn't reflected properly")
        }
    } else {
        panic!("Enum wasn't reflected properly")
    }
}

#[test]
fn test_wrong_ty() {
    let mut val = Value::from(Vec::<i32>::new());
    if let Type::Struct(info) = Type::from::<Struct>() {
        let field = &info.fields()[0];
        field
            .get_ref(&val)
            .expect_err("Successfully got Struct field on a Vec");
        field
            .set(&mut val, Value::from(-1.0))
            .expect_err("Successfully set Struct field on a Vec");
    }
}

#[test]
fn test_wrong_variant() {
    let mut val = Value::from(Enum::A);
    if let Type::Enum(ty) = val.ty() {
        if let Variant::Tuple(info) = &ty.variants()[1] {
            let field = &info.fields()[0];
            field
                .get_ref(&val)
                .expect_err("Successfully got field from unit enum");
            field
                .set(&mut val, Value::from(1.0))
                .expect_err("Successfully set field on unit enum");
        }
    } else {
        panic!("Enum wasn't reflected properly")
    }
}
