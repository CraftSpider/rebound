
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
        let inner = this.mut_borrow::<$ty>();
        inner.$($item)+ = value.cast();
    })
}
