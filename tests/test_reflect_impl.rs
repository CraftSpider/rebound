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
    let fns = Type::from::<Foo>().assoc_fns();
    assert_eq!(fns.len(), 3);
}

#[test]
fn test_new() {
    let new = &Type::from::<Foo>().assoc_fns()[0];

    assert_eq!(new.name(), "new");
    assert_eq!(new.assoc_ty(), Type::from::<Foo>());
    assert_eq!(new.arg_tys().len(), 0);
    assert_eq!(new.ret_ty(), Type::from::<Foo>());

    let val = new.call(None, vec![]).expect("Failed to call Foo::new");

    let foo = val.borrow::<Foo>();
    assert_eq!(foo.a, 1)
}

#[test]
fn test_get_a() {
    let foo = Value::from(Foo::new());
    // TODO: Value::as_ref to do this operation easily
    let foo_ref = Value::from(foo.borrow::<Foo>());

    let get_a = &Type::from::<Foo>().assoc_fns()[1];

    let a = get_a
        .call(Some(foo_ref), vec![])
        .expect("Failed to call Foo::get_a");

    assert_eq!(*a.borrow::<i32>(), 1);
}

#[test]
fn test_do_thing() {
    let mut foo = Value::from(Foo::new());
    // TODO: Value::as_mut to do this operation easily
    let foo_mut_ref = Value::from(foo.borrow_mut::<Foo>());

    let do_thing = &Type::from::<Foo>().assoc_fns()[2];

    do_thing
        .call(Some(foo_mut_ref), vec![])
        .expect("Failed to call Foo::do_thing");

    assert_eq!(foo.borrow::<Foo>().a, 2);
}
