
use rebound::Type;

#[test]
fn test_same_ty() {
    let int = 1;

    fn inner<'a>(_: &'a i32) {
        assert_eq!(Type::from::<&'static i32>(), Type::from::<&'a i32>())
    }

    inner(&int);
}
