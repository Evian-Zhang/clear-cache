#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;
#[cfg(any(target_os = "linux", target_os = "android"))]
use linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::*;

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
    os_arch_clear_cache(start, end)
}
