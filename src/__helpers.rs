
pub macro __make_static_helper($f:path $(, $args:ty)*) {
    #[allow(unused_mut, unused_variables)]
    Box::new(move |_, mut args| {
        $crate::Value::from( $f( $( args.remove(0).cast::<$args>(), )* ) )
    })
}

pub macro __make_dyn_helper($f:path, $ty:ty $(, $args:ty)*) {
    #[allow(unused_mut, unused_variables)]
    Box::new(move |this, mut args| {
        let s = this.unwrap();
        $crate::Value::from( $f( s.cast::<$ty>(), $( args.remove(0).cast::<$args>(), )* ) )
    })
}

pub macro __make_const_accessor($val:path) {
    Box::new(move || {
        $crate::Value::from($val)
    })
}

pub macro __make_ref_accessor($ty:ty, $($item:tt)+) {
    Box::new(|this| {
        let inner = this.borrow::<$ty>();
        $crate::Value::from_ref(&inner.$($item)+)
    })
}

pub macro __make_setter($ty:ty, $($item:tt)+) {
    Box::new(|this, value| {
        let inner = this.borrow_mut::<$ty>();
        inner.$($item)+ = value.cast();
    })
}

pub macro __make_enum_named_ref_accessor($ty:ty, $var:path, $item:ident) {
    Box::new(|this| {
        let inner = this.borrow::<$ty>();
        if let $var { $item } = inner {
            $crate::Value::from_ref($item)
        } else {
            unreachable!()
        }
    })
}

pub macro __make_enum_named_setter($ty:ty, $var:path, $item:ident) {
    Box::new(|this, value| {
        let inner = this.borrow_mut::<$ty>();
        if let $var { $item } = inner {
            *$item = value.cast();
        }
    })
}
