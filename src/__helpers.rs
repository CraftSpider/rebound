pub macro __make_const_accessor($val:path) {
    Box::new(move || $crate::Value::from($val))
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
