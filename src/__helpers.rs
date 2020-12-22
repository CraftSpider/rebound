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
