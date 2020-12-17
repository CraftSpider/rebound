
#![allow(dead_code)]

use rebound::{Type, FieldKind, rebound, VariantInfo};
use rebound::ty::CommonTypeInfo;

#[rebound]
enum Foo {
    Unit,
    Tuple(i32, char),
    Struct { bar: bool },
}

#[test]
fn test_foo_ty() {
    let ty = Type::from::<Foo>();

    assert_eq!(ty, Type::from::<Foo>());
    assert_eq!(ty.name(), "test_reflect_enum::Foo");

    if let Type::Enum(info) = ty {
        assert_eq!(info.variants().len(), 3);
    } else {
        assert!(false, "Reflected struct not a Type::Enum")
    }
}

#[test]
fn test_variant_unit() {
    if let Type::Enum(info) = Type::from::<Foo>() {
        let unit_var = info.variants()[0];
        if let VariantInfo::Unit(var) = unit_var {
            assert_eq!(var.name(), "Unit");
            assert_eq!(var.assoc_ty(), Type::from::<Foo>());
        }
    }
}

#[test]
fn test_variant_tuple() {
    if let Type::Enum(info) = Type::from::<Foo>() {
        let tuple_var = info.variants()[1];
        if let VariantInfo::Tuple(var) = tuple_var {
            assert_eq!(var.name(), "Tuple");
            assert_eq!(var.fields().len(), 2);
            assert_eq!(var.assoc_ty(), Type::from::<Foo>());
        }
    }
}

#[test]
fn test_field_tuple() {
    if let Type::Enum(info) = Type::from::<Foo>() {
        if let VariantInfo::Tuple(var) = info.variants()[1] {
            let field_1 = &var.fields()[0];
            if let FieldKind::EnumTuple { idx, assoc_var } = field_1.kind() {
                assert_eq!(*idx, 0);
                assert_eq!(*assoc_var, info.variants()[1]);
            } else {
                assert!(false, "Reflected field not a FieldKind::EnumTuple")
            }
            assert_eq!(field_1.assoc_ty(), Type::from::<Foo>());
            assert_eq!(field_1.ty(), Type::from::<i32>());

            let field_2 = &var.fields()[1];
            if let FieldKind::EnumTuple { idx, assoc_var } = field_2.kind() {
                assert_eq!(*idx, 1);
                assert_eq!(*assoc_var, info.variants()[1]);
            } else {
                assert!(false, "Reflected field not a FieldKind::EnumTuple")
            }
            assert_eq!(field_2.assoc_ty(), Type::from::<Foo>());
            assert_eq!(field_2.ty(), Type::from::<char>());
        }
    }
}

#[test]
fn test_field_struct() {
    if let Type::Enum(info) = Type::from::<Foo>() {
        if let VariantInfo::Struct(var) = info.variants()[2] {
            let field_1 = &var.fields()[0];
            if let FieldKind::EnumNamed { name, assoc_var } = field_1.kind() {
                assert_eq!(*name, "bar");
                assert_eq!(*assoc_var, info.variants()[2]);
            } else {
                assert!(false, "Reflected field not a FieldKind::EnumTuple")
            }
            assert_eq!(field_1.assoc_ty(), Type::from::<Foo>());
            assert_eq!(field_1.ty(), Type::from::<bool>());
        }
    }
}

#[test]
fn test_variant_struct() {
    if let Type::Enum(info) = Type::from::<Foo>() {
        let tuple_var = info.variants()[2];
        if let VariantInfo::Tuple(var) = tuple_var {
            assert_eq!(var.name(), "Struct");
            assert_eq!(var.fields().len(), 1);
            assert_eq!(var.assoc_ty(), Type::from::<Foo>());
        }
    }
}
