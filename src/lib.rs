#![no_std]

//! Taken from https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/clear_cache.c

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::*;

pub unsafe fn clear_cache<T>(start: *const T, end: *const T) -> bool {
    os_arch_clear_cache(start, end)
}
