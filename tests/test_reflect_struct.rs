use rebound::ty::CommonTypeInfo;
use rebound::{rebound, FieldKind, Type};

#[rebound]
struct Foo {
    a: i32,
    b: &'static str,
}

#[test]
fn test_struct_ty() {
    let ty = Type::of::<Foo>();

    assert_eq!(ty, Type::of::<Foo>());
    assert_eq!(ty.name(), "test_reflect_struct::Foo");

    if let Type::Struct(info) = ty {
        assert_eq!(info.fields().len(), 2);
    } else {
        assert!(false, "Reflected struct not a Type::Struct")
    }
}

#[test]
fn test_field_a() {
    if let Type::Struct(info) = Type::of::<Foo>() {
        let field_a = &info.fields()[0];
        if let FieldKind::Named { name } = field_a.kind() {
            assert_eq!(*name, "a")
        } else {
            assert!(false, "Reflected field not a FieldKind::Named")
        }
        assert_eq!(field_a.assoc_ty(), Type::of::<Foo>());
        assert_eq!(field_a.ty(), Type::of::<i32>());
    }
}

#[test]
fn test_field_b() {
    if let Type::Struct(info) = Type::of::<Foo>() {
        let field_b = &info.fields()[1];
        if let FieldKind::Named { name } = field_b.kind() {
            assert_eq!(*name, "b")
        } else {
            assert!(false, "Reflected field not a FieldKind::Named")
        }
        assert_eq!(field_b.assoc_ty(), Type::of::<Foo>());
        assert_eq!(field_b.ty(), Type::of::<&'static str>());
    }
}
