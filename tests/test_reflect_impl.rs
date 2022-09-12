#![allow(incomplete_features)]
#![feature(specialization)]

use rebound::ty::CommonTypeInfo;
use rebound::{rebound, Type, Value};

#[rebound]
struct Foo {
    a: i32,
}

#[rebound]
impl Foo {
    const FOO: Foo = Foo { a: -1 };

    fn new() -> Foo {
        Foo { a: 1 }
    }

    fn get_a(&self) -> i32 {
        self.a
    }

    fn do_thing(&mut self) {
        self.a += 1;
    }
}

#[test]
fn test_assoc_fns() {
    let ty = Type::of::<Foo>();

    let consts = ty.assoc_consts();
    assert_eq!(consts.len(), 1);
    let fns = ty.assoc_fns();
    assert_eq!(fns.len(), 3);
}

#[test]
fn test_const() {
    let foo = &Type::of::<Foo>().assoc_consts()[0];

    assert_eq!(foo.name(), "FOO");
    assert_eq!(foo.assoc_ty(), Type::of::<Foo>());
    assert_eq!(foo.ty(), Type::of::<Foo>());

    let val = foo.get().unwrap();
    assert_eq!(val.borrow::<Foo>().a, -1);
}

#[test]
fn test_new() {
    let new = &Type::of::<Foo>().assoc_fns()[0];

    assert_eq!(new.name(), "new");
    assert_eq!(new.assoc_ty(), Type::of::<Foo>());
    assert_eq!(new.arg_tys().len(), 0);
    assert_eq!(new.ret_ty(), Type::of::<Foo>());

    let val = new.call(None, vec![]).expect("Failed to call Foo::new");

    let foo = val.borrow::<Foo>();
    assert_eq!(foo.a, 1)
}

#[test]
fn test_get_a() {
    let foo = Value::from(Foo::new());
    let foo_ref = foo.as_ref().unwrap();

    let get_a = &Type::of::<Foo>().assoc_fns()[1];

    let a = get_a
        .call(Some(foo_ref), vec![])
        .expect("Failed to call Foo::get_a");

    assert_eq!(*a.borrow::<i32>(), 1);
}

#[test]
fn test_do_thing() {
    let mut foo = Value::from(Foo::new());
    let foo_mut_ref = foo.as_mut().unwrap();

    let do_thing = &Type::of::<Foo>().assoc_fns()[2];

    do_thing
        .call(Some(foo_mut_ref), vec![])
        .expect("Failed to call Foo::do_thing");

    assert_eq!(foo.borrow::<Foo>().a, 2);
}
