use rebound::Type;

#[test]
fn test_same_ty() {
    let int = 1;

    fn inner<'a>(_: &'a i32) {
        assert_eq!(Type::of::<&'static i32>(), Type::of::<&'a i32>())
    }

    inner(&int);
}
