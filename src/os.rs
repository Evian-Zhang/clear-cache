//! Implementations of os_arch_clear_cache for architectures where it is (or may be) a
//! privileged operation that requires calling into the operating system.

#[cfg_attr(any(target_os = "linux", target_os = "android"), path = "linux.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
#[cfg_attr(target_os = "none", path = "arch.rs")]
mod clear_cache_impl;

pub(crate) use clear_cache_impl::os_arch_clear_cache;
