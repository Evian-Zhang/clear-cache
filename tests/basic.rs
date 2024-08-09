fn foo(a: usize) -> usize {
    a + 1
}

#[test]
fn basic_test() {
    let is_success =
        unsafe { clear_cache::clear_cache(&foo, (&foo as *const _ as *const u8).add(4).cast()) };
    assert!(is_success)
}
