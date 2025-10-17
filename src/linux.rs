#[cfg(target_arch = "arm")]
pub(crate) unsafe fn os_arch_clear_cache<T>(start: *const T, end: *const T) -> bool {
    const __ARM_NR_CACHEFLUSH: i32 = 0x0f0002;
    let res = libc::syscall(__ARM_NR_CACHEFLUSH, start as usize, end as usize, 0);
    res == 0
}

#[cfg(target_arch = "riscv64")]
pub(crate) unsafe fn os_arch_clear_cache<T>(start: *const T, end: *const T) -> bool {
    const __NR_RISCV_FLUSH_ICACHE: i64 = 259;
    let res = unsafe {
        libc::syscall(
            __NR_RISCV_FLUSH_ICACHE,
            start as usize as u64,
            end as usize as u64,
            // "0" means that we clear cache for all threads (SYS_RISCV_FLUSH_ICACHE_ALL)
            0,
        )
    };
    res == 0
}

// On aarch64 linux, there is no cache invalidation syscall. Use the asm implementation.
#[cfg(target_arch = "aarch64")]
mod arch;
#[cfg(target_arch = "aarch64")]
pub(crate) use arch::os_arch_clear_cache;
