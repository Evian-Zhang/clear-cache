extern "C" {
    // libkern/OSCacheControl.h
    // void	sys_icache_invalidate( void *start, size_t len) __OSX_AVAILABLE_STARTING(__MAC_10_5, __IPHONE_2_0);
    fn sys_icache_invalidate(start: *mut core::ffi::c_void, len: usize);
}

pub(crate) unsafe fn os_arch_clear_cache<T>(start: *const T, end: *const T) -> bool {
    sys_icache_invalidate(
        start.cast(),
        (end as *const u8).offset_from(start as *const u8) as usize,
    );
    true
}
