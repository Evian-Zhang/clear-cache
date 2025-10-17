#![doc = include_str!("../README.md")]
#![no_std]

#[cfg_attr(
    any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "loongarch64",
    ),
    path = "arch.rs"
)]
#[cfg_attr(
    not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "loongarch64",
    )),
    path = "os.rs"
)]
mod clear_cache_impl;

/// Flush CPU's instruction cache at given range.
///
/// Return `false` if the cache is not successfully cleared.
///
/// # Safety
///
/// It seems that this function should be safe. However, the complexity of certain
/// instructions and syscalls make it difficult to guarantee that this function is totally
/// safe.
pub unsafe fn clear_cache<T>(start: *const T, end: *const T) -> bool {
    clear_cache_impl::os_arch_clear_cache(start, end)
}
