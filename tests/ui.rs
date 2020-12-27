#[cfg_attr(miri, ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("ui/fail/**/*.rs");
    t.pass("ui/pass/**/*.rs");
}
