fn foo(a: usize) -> usize {
    a + 1
}

#[test]
fn basic_test() {
    let is_success =
        unsafe { clear_cache::clear_cache(foo as *const (), (foo as *const ()).add(4)) };
    assert!(is_success)
}
