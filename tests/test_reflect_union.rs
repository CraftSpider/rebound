use rebound::ty::CommonTypeInfo;
use rebound::{rebound, Type};

#[rebound]
union Foo {
    a: i32,
    b: f32,
}

#[test]
fn test_union_ty() {
    let ty = Type::of::<Foo>();

    assert_eq!(ty, Type::of::<Foo>());
    assert_eq!(ty.name(), "test_reflect_union::Foo");

    if let Type::Union(info) = ty {
        assert_eq!(info.fields().len(), 2);
    } else {
        assert!(false, "Reflected union not a Type::Union")
    }
}

#[test]
fn test_field_a() {
    if let Type::Union(info) = Type::of::<Foo>() {
        let field_a = &info.fields()[0];
        assert_eq!(field_a.name(), "a");
        assert_eq!(field_a.assoc_ty(), Type::of::<Foo>());
        assert_eq!(field_a.ty(), Type::of::<i32>());
    }
}

#[test]
fn test_field_b() {
    if let Type::Union(info) = Type::of::<Foo>() {
        let field_b = &info.fields()[1];
        assert_eq!(field_b.name(), "b");
        assert_eq!(field_b.assoc_ty(), Type::of::<Foo>());
        assert_eq!(field_b.ty(), Type::of::<f32>());
    }
}
